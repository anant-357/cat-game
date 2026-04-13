#!/usr/bin/env python3
"""
Procedural cat model generator for the Bevy cat-game.

Creates a low-poly animated cat (Idle / Walk / Run) and exports it as a
binary GLTF (.glb) suitable for Bevy 0.18's asset loader.

Usage:
    blender --background --python generate_cat.py

Output:
    assets/models/cat.glb  (relative to this script)

Requirements:
    Blender 3.6+ or 4.x  (tested on 4.x)
"""

import bpy
import bmesh
import math
import os
from mathutils import Vector, Euler

# ── Output path ──────────────────────────────────────────────────────────────

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_PATH = os.path.join(SCRIPT_DIR, "assets", "models", "cat.glb")

# ── Helpers ───────────────────────────────────────────────────────────────────

def deselect_all():
    bpy.ops.object.select_all(action='DESELECT')


def set_active(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)


def obj_mode():
    if bpy.context.mode != 'OBJECT':
        bpy.ops.object.mode_set(mode='OBJECT')


# ── Scene setup ───────────────────────────────────────────────────────────────

def clear_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    for d in (bpy.data.meshes, bpy.data.armatures,
              bpy.data.materials, bpy.data.curves,
              bpy.data.actions):
        for item in list(d):
            d.remove(item)


# ── Material ──────────────────────────────────────────────────────────────────

def create_material():
    """Golden-brown PBR material to match the existing cuboid color."""
    mat = bpy.data.materials.new("CatMaterial")
    mat.use_nodes = True
    tree = mat.node_tree
    tree.nodes.clear()

    bsdf = tree.nodes.new("ShaderNodeBsdfPrincipled")
    bsdf.inputs["Base Color"].default_value = (122 / 255, 100 / 255, 21 / 255, 1.0)
    bsdf.inputs["Roughness"].default_value = 0.85
    bsdf.inputs["Metallic"].default_value = 0.0

    out = tree.nodes.new("ShaderNodeOutputMaterial")
    out.location = (300, 0)
    tree.links.new(bsdf.outputs["BSDF"], out.inputs["Surface"])
    return mat


# ── Mesh ──────────────────────────────────────────────────────────────────────

def _add_sphere(loc, radius, sx=1, sy=1, sz=1, seg=10, rings=7):
    bpy.ops.mesh.primitive_uv_sphere_add(
        segments=seg, ring_count=rings, radius=radius, location=loc)
    obj = bpy.context.active_object
    obj.scale = (sx, sy, sz)
    bpy.ops.object.transform_apply(scale=True)
    obj.select_set(False)
    return obj


def _add_cylinder(loc, radius, depth, verts=8, rx=0, ry=0, rz=0):
    bpy.ops.mesh.primitive_cylinder_add(
        vertices=verts, radius=radius, depth=depth, location=loc,
        rotation=(rx, ry, rz))
    obj = bpy.context.active_object
    bpy.ops.object.transform_apply(rotation=True)
    obj.select_set(False)
    return obj


def _add_cone(loc, radius, depth, verts=4, rx=0, ry=0, rz=0):
    bpy.ops.mesh.primitive_cone_add(
        vertices=verts, radius1=radius, radius2=0, depth=depth,
        location=loc, rotation=(rx, ry, rz))
    obj = bpy.context.active_object
    bpy.ops.object.transform_apply(rotation=True)
    obj.select_set(False)
    return obj


def create_cat_mesh():
    """
    Build the cat geometry from primitives and merge into one object.

    Blender coordinate frame: Z-up, cat faces +Y.
    Paws rest on the Z=0 plane.  All units are metres (≈ 1 m tall with head).
    """
    deselect_all()
    parts = []

    # Body ─ elongated ellipsoid
    parts.append(_add_sphere((0, 0, 0.40), 0.33, sx=0.72, sy=1.10, sz=0.65))

    # Head ─ slightly flattened sphere at front of body
    parts.append(_add_sphere((0, 0.74, 0.62), 0.27, sx=1.05, sy=0.92, sz=0.95))

    # Snout ─ small protrusion forward from head
    parts.append(_add_sphere((0, 0.97, 0.57), 0.105, sx=1.0, sy=0.68, sz=0.80))

    # Ears (4-sided cones, angled slightly outward)
    parts.append(_add_cone((0.16, 0.75, 0.91), 0.09, 0.22, verts=4))
    parts.append(_add_cone((-0.16, 0.75, 0.91), 0.09, 0.22, verts=4))

    # Front legs
    parts.append(_add_cylinder((0.22, 0.54, 0.20), 0.075, 0.40))
    parts.append(_add_cylinder((-0.22, 0.54, 0.20), 0.075, 0.40))

    # Back legs (slightly thicker)
    parts.append(_add_cylinder((0.22, -0.52, 0.20), 0.085, 0.40))
    parts.append(_add_cylinder((-0.22, -0.52, 0.20), 0.085, 0.40))

    # Paws ─ flattened spheres at the foot of each leg
    for (px, py) in [(0.22, 0.54), (-0.22, 0.54), (0.22, -0.52), (-0.22, -0.52)]:
        parts.append(_add_sphere((px, py, 0.03), 0.09, sx=1.1, sy=1.3, sz=0.55))

    # Tail ─ angled cylinder + tapered tip
    angle = math.radians(48)
    parts.append(_add_cylinder((0, -0.84, 0.46), 0.045, 0.55, verts=8, rx=angle))
    parts.append(_add_cone((0, -1.09, 0.74), 0.04, 0.22, verts=8, rx=angle))

    # Join all parts into one object
    for p in parts:
        p.select_set(True)
    bpy.context.view_layer.objects.active = parts[0]
    bpy.ops.object.join()

    cat = bpy.context.active_object
    cat.name = "Cat"
    cat.data.name = "CatMesh"

    # Merge vertices at seams (where primitives overlap)
    bpy.ops.object.mode_set(mode='EDIT')
    bpy.ops.mesh.select_all(action='SELECT')
    bpy.ops.mesh.remove_doubles(threshold=0.005)
    bpy.ops.object.mode_set(mode='OBJECT')

    bpy.ops.object.shade_smooth()

    mat = create_material()
    cat.data.materials.append(mat)
    for poly in cat.data.polygons:
        poly.material_index = 0

    return cat


# ── Armature ──────────────────────────────────────────────────────────────────

def create_armature():
    """
    Hierarchy:
        Root
        └─ Hip
           ├─ Spine ─ Neck ─ Head
           ├─ Tail.1 ─ Tail.2 ─ Tail.3
           ├─ FrontLeg.L.Upper ─ FrontLeg.L.Lower ─ FrontLeg.L.Paw
           ├─ FrontLeg.R.Upper ─ FrontLeg.R.Lower ─ FrontLeg.R.Paw
           ├─ BackLeg.L.Upper  ─ BackLeg.L.Lower  ─ BackLeg.L.Paw
           └─ BackLeg.R.Upper  ─ BackLeg.R.Lower  ─ BackLeg.R.Paw
    """
    arm_data = bpy.data.armatures.new("CatArmature")
    arm_obj = bpy.data.objects.new("CatRig", arm_data)
    bpy.context.scene.collection.objects.link(arm_obj)

    obj_mode()
    deselect_all()
    set_active(arm_obj)
    bpy.ops.object.mode_set(mode='EDIT')

    eb = arm_data.edit_bones

    def bone(name, head, tail, parent=None, connect=False):
        b = eb.new(name)
        b.head = Vector(head)
        b.tail = Vector(tail)
        if parent:
            b.parent = eb[parent]
            b.use_connect = connect
        return b

    # Root & Hip
    bone("Root", (0, 0, 0),    (0, 0, 0.12))
    bone("Hip",  (0, 0, 0.40), (0, 0, 0.56), parent="Root")

    # Spine chain
    bone("Spine", (0, 0,    0.42), (0, 0.46, 0.54), parent="Hip")
    bone("Neck",  (0, 0.46, 0.54), (0, 0.63, 0.66), parent="Spine")
    bone("Head",  (0, 0.63, 0.66), (0, 0.82, 0.72), parent="Neck")

    # Tail chain
    bone("Tail.1", (0, -0.56, 0.42), (0, -0.76, 0.57), parent="Hip")
    bone("Tail.2", (0, -0.76, 0.57), (0, -0.93, 0.70), parent="Tail.1")
    bone("Tail.3", (0, -0.93, 0.70), (0, -1.06, 0.82), parent="Tail.2")

    # Legs
    for side, x in [("L", 0.22), ("R", -0.22)]:
        for prefix, by in [("FrontLeg", 0.54), ("BackLeg", -0.52)]:
            bone(f"{prefix}.{side}.Upper", (x, by, 0.38), (x, by, 0.20), parent="Hip")
            bone(f"{prefix}.{side}.Lower", (x, by, 0.20), (x, by, 0.05),
                 parent=f"{prefix}.{side}.Upper")
            bone(f"{prefix}.{side}.Paw",   (x, by, 0.05), (x, by + 0.10, 0.01),
                 parent=f"{prefix}.{side}.Lower")

    bpy.ops.object.mode_set(mode='OBJECT')
    return arm_obj


# ── Skinning ──────────────────────────────────────────────────────────────────

def skin_mesh(cat_obj, arm_obj):
    obj_mode()
    deselect_all()
    cat_obj.select_set(True)
    arm_obj.select_set(True)
    set_active(arm_obj)
    bpy.ops.object.parent_set(type='ARMATURE_AUTO')


# ── Animation helpers ─────────────────────────────────────────────────────────

def _kf(arm_obj, bone_name, frame, rot_xyz):
    """Insert a rotation_euler keyframe on a pose bone."""
    pb = arm_obj.pose.bones[bone_name]
    pb.rotation_mode = 'XYZ'
    pb.rotation_euler = Euler(rot_xyz, 'XYZ')
    pb.keyframe_insert(data_path="rotation_euler", frame=frame)


def _kf_loc(arm_obj, bone_name, frame, loc_xyz):
    """Insert a location keyframe on a pose bone."""
    pb = arm_obj.pose.bones[bone_name]
    pb.location = Vector(loc_xyz)
    pb.keyframe_insert(data_path="location", frame=frame)


def _reset_pose(arm_obj):
    """Set all bones to rest position (no keyframe)."""
    for pb in arm_obj.pose.bones:
        pb.rotation_mode = 'XYZ'
        pb.rotation_euler = Euler((0, 0, 0), 'XYZ')
        pb.location = Vector((0, 0, 0))


def _make_cyclic(action):
    # Blender 4.4+ moved fcurves into slotted action layers; try both APIs.
    try:
        fcurves = action.fcurves  # legacy (< 4.4)
    except AttributeError:
        fcurves = []
        for layer in getattr(action, 'layers', []):
            for strip in getattr(layer, 'strips', []):
                for cb in getattr(strip, 'channelbags', []):
                    fcurves.extend(cb.fcurves)
    for fc in fcurves:
        mod = fc.modifiers.new(type='CYCLES')
        mod.mode_before = 'REPEAT'
        mod.mode_after = 'REPEAT'


def _push_to_nla(arm_obj, action, strip_name, start_frame=1):
    """Push action to NLA as a named strip (becomes GLTF animation name)."""
    track = arm_obj.animation_data.nla_tracks.new()
    track.name = strip_name
    strip = track.strips.new(strip_name, start_frame, action)
    strip.name = strip_name
    return strip


# ── Idle animation ────────────────────────────────────────────────────────────

def create_idle_action(arm_obj):
    """24 frames: gentle breathing + tail sway + slight head bob."""
    action = bpy.data.actions.new("Idle")
    arm_obj.animation_data.action = action

    _reset_pose(arm_obj)

    # Spine ─ subtle breathing
    for fr, rx in [(1, 0.0), (12, 0.04), (24, 0.0)]:
        _kf(arm_obj, "Spine", fr, (rx, 0, 0))

    # Tail ─ gentle side sway
    for fr, rz in [(1, 0.0), (8, 0.18), (16, -0.18), (24, 0.0)]:
        _kf(arm_obj, "Tail.1", fr, (0.05, 0, rz))
        _kf(arm_obj, "Tail.2", fr, (0.05, 0, rz * 1.4))
        _kf(arm_obj, "Tail.3", fr, (0.05, 0, rz * 2.0))

    # Head ─ slow nod
    for fr, rx in [(1, 0.0), (12, -0.04), (24, 0.0)]:
        _kf(arm_obj, "Head", fr, (rx, 0, 0))

    # All other bones stay at rest
    rest_bones = [
        "Root", "Hip", "Neck",
        "FrontLeg.L.Upper", "FrontLeg.L.Lower", "FrontLeg.L.Paw",
        "FrontLeg.R.Upper", "FrontLeg.R.Lower", "FrontLeg.R.Paw",
        "BackLeg.L.Upper",  "BackLeg.L.Lower",  "BackLeg.L.Paw",
        "BackLeg.R.Upper",  "BackLeg.R.Lower",  "BackLeg.R.Paw",
    ]
    for name in rest_bones:
        for fr in [1, 24]:
            _kf(arm_obj, name, fr, (0, 0, 0))

    _make_cyclic(action)
    return action


# ── Walk animation ────────────────────────────────────────────────────────────

def create_walk_action(arm_obj):
    """
    16 frames: diagonal gait (FL+RR, then FR+BL).
    In-place — no root translation.
    """
    action = bpy.data.actions.new("Walk")
    arm_obj.animation_data.action = action

    _reset_pose(arm_obj)

    # Spine side-sway (opposite to leading leg)
    for fr, rz in [(1, 0.0), (5, 0.07), (9, 0.0), (13, -0.07), (16, 0.0)]:
        _kf(arm_obj, "Spine", fr, (0, 0, rz))

    # Hip vertical bob
    for fr, z in [(1, 0.0), (5, -0.025), (9, 0.0), (13, -0.025), (16, 0.0)]:
        _kf_loc(arm_obj, "Hip", fr, (0, 0, z))

    # Helper: (frame, upper_rx, lower_rx) tables for each phase
    # Positive rx = leg swings forward; negative = back
    FL_keys = [(1, -0.28, 0.22), (5,  0.08, -0.06), (9,  0.28, -0.18), (13,  0.04, 0.12), (16, -0.28, 0.22)]
    FR_keys = [(1,  0.28, -0.18), (5,  0.04, 0.12), (9, -0.28,  0.22), (13,  0.08, -0.06), (16,  0.28, -0.18)]
    BL_keys = [(1,  0.22, -0.14), (5,  0.04, 0.09), (9, -0.22,  0.16), (13,  0.06, -0.05), (16,  0.22, -0.14)]
    BR_keys = [(1, -0.22,  0.16), (5,  0.06, -0.05), (9,  0.22, -0.14), (13,  0.04, 0.09), (16, -0.22,  0.16)]

    for leg, keys in [
        ("FrontLeg.L", FL_keys), ("FrontLeg.R", FR_keys),
        ("BackLeg.L",  BL_keys), ("BackLeg.R",  BR_keys),
    ]:
        for (fr, urx, lrx) in keys:
            _kf(arm_obj, f"{leg}.Upper", fr, (urx, 0, 0))
            _kf(arm_obj, f"{leg}.Lower", fr, (lrx, 0, 0))
            _kf(arm_obj, f"{leg}.Paw",   fr, (0, 0, 0))

    # Tail ─ light counter-sway
    for fr, rz in [(1, 0.12), (9, -0.12), (16, 0.12)]:
        _kf(arm_obj, "Tail.1", fr, (0.08, 0, rz))
        _kf(arm_obj, "Tail.2", fr, (0.08, 0, rz * 1.5))
        _kf(arm_obj, "Tail.3", fr, (0.08, 0, rz * 2.0))

    # Neck/head ─ slight forward nod on stride
    for fr, rx in [(1, 0.0), (5, 0.06), (9, 0.0), (13, 0.06), (16, 0.0)]:
        _kf(arm_obj, "Neck", fr, (rx, 0, 0))
        _kf(arm_obj, "Head", fr, (rx * 0.5, 0, 0))

    for fr in [1, 16]:
        _kf(arm_obj, "Root", fr, (0, 0, 0))

    _make_cyclic(action)
    return action


# ── Run animation ─────────────────────────────────────────────────────────────

def create_run_action(arm_obj):
    """
    12 frames: bound/gallop — both front legs together, then both back legs.
    Spine flexes dramatically.  In-place.
    """
    action = bpy.data.actions.new("Run")
    arm_obj.animation_data.action = action

    _reset_pose(arm_obj)

    # Spine ─ strong flex/extend (backbone pumping)
    for fr, rx in [(1, -0.28), (4, 0.0), (7,  0.35), (10, 0.0), (12, -0.28)]:
        _kf(arm_obj, "Spine", fr, (rx, 0, 0))

    # Hip bob
    for fr, z in [(1, 0.05), (4, 0.0), (7, -0.05), (10, 0.0), (12, 0.05)]:
        _kf_loc(arm_obj, "Hip", fr, (0, 0, z))

    # Both front legs ─ reach at frame 1, push at frame 7
    for side in ("L", "R"):
        fl = f"FrontLeg.{side}"
        for (fr, urx, lrx) in [
            (1,  -0.38,  0.28),   # reach forward
            (4,   0.06, -0.05),   # stance
            (7,   0.32, -0.22),   # push back
            (10,  0.05,  0.12),   # fold/lift
            (12, -0.38,  0.28),   # reach (= frame 1)
        ]:
            _kf(arm_obj, f"{fl}.Upper", fr, (urx, 0, 0))
            _kf(arm_obj, f"{fl}.Lower", fr, (lrx, 0, 0))
            _kf(arm_obj, f"{fl}.Paw",   fr, (0, 0, 0))

    # Both back legs ─ offset 6 frames from front
    for side in ("L", "R"):
        bl = f"BackLeg.{side}"
        for (fr, urx, lrx) in [
            (1,   0.32, -0.22),   # push back
            (4,   0.05,  0.12),   # fold/lift
            (7,  -0.38,  0.28),   # reach forward
            (10,  0.06, -0.05),   # stance
            (12,  0.32, -0.22),   # push back (= frame 1)
        ]:
            _kf(arm_obj, f"{bl}.Upper", fr, (urx, 0, 0))
            _kf(arm_obj, f"{bl}.Lower", fr, (lrx, 0, 0))
            _kf(arm_obj, f"{bl}.Paw",   fr, (0, 0, 0))

    # Tail ─ streams behind during gallop
    for fr, rx in [(1, 0.35), (7, -0.05), (12, 0.35)]:
        _kf(arm_obj, "Tail.1", fr, (rx, 0, 0))
        _kf(arm_obj, "Tail.2", fr, (rx * 0.75, 0, 0))
        _kf(arm_obj, "Tail.3", fr, (rx * 0.45, 0, 0))

    # Neck/head ─ nods with spine
    for fr, nx, hx in [(1, -0.12, -0.06), (7,  0.18, 0.10), (12, -0.12, -0.06)]:
        _kf(arm_obj, "Neck", fr, (nx, 0, 0))
        _kf(arm_obj, "Head", fr, (hx, 0, 0))

    for fr in [1, 12]:
        _kf(arm_obj, "Root", fr, (0, 0, 0))

    _make_cyclic(action)
    return action


# ── Create all animations ─────────────────────────────────────────────────────

def create_animations(arm_obj):
    obj_mode()
    deselect_all()
    set_active(arm_obj)
    bpy.ops.object.mode_set(mode='POSE')

    arm_obj.animation_data_create()

    idle_action = create_idle_action(arm_obj)
    walk_action = create_walk_action(arm_obj)
    run_action  = create_run_action(arm_obj)

    # Clear the active action so we don't accidentally export it again
    arm_obj.animation_data.action = None

    # Push each action into the NLA editor as a named strip.
    # Blender's GLTF exporter reads NLA strip names as animation names.
    _push_to_nla(arm_obj, idle_action, "Idle", start_frame=1)
    _push_to_nla(arm_obj, walk_action, "Walk", start_frame=1)
    _push_to_nla(arm_obj, run_action,  "Run",  start_frame=1)

    bpy.ops.object.mode_set(mode='OBJECT')


# ── Export ────────────────────────────────────────────────────────────────────

def export_glb(path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    deselect_all()
    obj_mode()

    bpy.ops.export_scene.gltf(
        filepath=path,
        export_format='GLB',
        # Axis conversion: Blender Z-up → GLTF/Bevy Y-up
        export_yup=True,
        # Animations
        export_animations=True,
        export_nla_strips=True,       # each NLA strip → separate GLTF animation
        export_nla_strips_merged_animation_name="",  # keep strips separate
        export_action_filter=False,
        # Mesh / skin
        export_skins=True,
        export_apply=False,
        export_morph=False,
        # No extras
        export_cameras=False,
        export_lights=False,
    )
    print(f"[cat-gen] Wrote {path}")


# ── Entry point ───────────────────────────────────────────────────────────────

def main():
    print("[cat-gen] Starting cat model generation …")

    clear_scene()

    print("[cat-gen] Building mesh …")
    cat_obj = create_cat_mesh()

    print("[cat-gen] Building armature …")
    arm_obj = create_armature()

    print("[cat-gen] Skinning (auto-weights) …")
    skin_mesh(cat_obj, arm_obj)

    print("[cat-gen] Creating animations …")
    create_animations(arm_obj)

    print("[cat-gen] Exporting GLB …")
    export_glb(OUTPUT_PATH)

    print("[cat-gen] Done.")


main()
