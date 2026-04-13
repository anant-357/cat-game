#!/usr/bin/env python3
"""
Synthesize placeholder audio assets for Stray Embers.

Requires: numpy, scipy
    pip install numpy scipy

Output: assets/audio/{footstep,ambient,interact,ember_light,win}.wav
"""

import os
import numpy as np
from scipy.io import wavfile
from scipy.signal import butter, lfilter

SAMPLE_RATE = 44100
OUTPUT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets", "audio")


def write_wav(name: str, samples: np.ndarray):
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    # Normalize and convert to int16
    peak = np.max(np.abs(samples))
    if peak > 0:
        samples = samples / peak * 0.85
    out = (samples * 32767).astype(np.int16)
    path = os.path.join(OUTPUT_DIR, name)
    wavfile.write(path, SAMPLE_RATE, out)
    print(f"  wrote {path}")


def seconds(s: float) -> int:
    return int(SAMPLE_RATE * s)


def envelope(n: int, attack: float = 0.005, decay: float = 0.1, sustain: float = 0.7, release: float = 0.1) -> np.ndarray:
    """Simple ADSR envelope (proportional to n samples)."""
    a = int(n * attack)
    d = int(n * decay)
    r = int(n * release)
    s_len = n - a - d - r
    env = np.concatenate([
        np.linspace(0, 1, a),
        np.linspace(1, sustain, d),
        np.full(max(s_len, 0), sustain),
        np.linspace(sustain, 0, r),
    ])
    return env[:n]


def bandpass(signal: np.ndarray, lo: float, hi: float) -> np.ndarray:
    b, a = butter(4, [lo / (SAMPLE_RATE / 2), hi / (SAMPLE_RATE / 2)], btype="band")
    return lfilter(b, a, signal)


# ── Footstep ──────────────────────────────────────────────────────────────────

def make_footstep():
    """Short percussive click: band-limited noise burst with fast decay."""
    n = seconds(0.08)
    noise = np.random.randn(n)
    filtered = bandpass(noise, 800, 3500)
    env = np.exp(-np.linspace(0, 18, n))
    write_wav("footstep.wav", filtered * env * 0.9)


# ── Ambient ───────────────────────────────────────────────────────────────────

def make_ambient():
    """3-second low-frequency cave drone with slow amplitude modulation."""
    n = seconds(3.0)
    t = np.linspace(0, 3.0, n)

    # Root drone at 55 Hz with harmonics
    sig  = 0.60 * np.sin(2 * np.pi * 55   * t)
    sig += 0.25 * np.sin(2 * np.pi * 110  * t + 0.3)
    sig += 0.10 * np.sin(2 * np.pi * 165  * t + 1.1)
    sig += 0.05 * np.sin(2 * np.pi * 220  * t + 0.7)

    # Add subtle high-frequency cave hiss
    hiss = bandpass(np.random.randn(n), 2000, 6000) * 0.04

    # Slow tremolo (0.3 Hz)
    tremolo = 0.85 + 0.15 * np.sin(2 * np.pi * 0.3 * t)

    # Fade in/out for seamless loop
    fade = np.ones(n)
    fade_len = seconds(0.15)
    fade[:fade_len] = np.linspace(0, 1, fade_len)
    fade[-fade_len:] = np.linspace(1, 0, fade_len)

    write_wav("ambient.wav", (sig + hiss) * tremolo * fade)


# ── Interact ──────────────────────────────────────────────────────────────────

def make_interact():
    """150ms rising sine sweep — bright interaction chime."""
    n = seconds(0.15)
    t = np.linspace(0, 0.15, n)
    freq = np.linspace(350, 950, n)
    phase = np.cumsum(2 * np.pi * freq / SAMPLE_RATE)
    sig = np.sin(phase)
    env = np.exp(-np.linspace(0, 8, n))
    write_wav("interact.wav", sig * env)


# ── Ember light ───────────────────────────────────────────────────────────────

def make_ember_light():
    """300ms crackling noise + 200ms warm tone — fire ignition."""
    # Crackle: band-passed noise with fast random amplitude modulation
    n_crackle = seconds(0.30)
    t_c = np.linspace(0, 0.30, n_crackle)
    noise = bandpass(np.random.randn(n_crackle), 400, 3000)
    crackle_env = np.exp(-np.linspace(0, 6, n_crackle))
    # Random pops
    pops = np.random.choice([0.0, 1.0], size=n_crackle, p=[0.97, 0.03])
    crackle = (noise + pops * 0.5) * crackle_env

    # Warm tone: 220 Hz sine fade-in
    n_tone = seconds(0.20)
    t_t = np.linspace(0, 0.20, n_tone)
    tone = np.sin(2 * np.pi * 220 * t_t) * np.linspace(0, 0.6, n_tone)

    sig = np.concatenate([crackle, tone])
    write_wav("ember_light.wav", sig)


# ── Win ───────────────────────────────────────────────────────────────────────

def make_win():
    """600ms ascending arpeggio: C4 E4 G4 C5."""
    notes = [261.63, 329.63, 392.00, 523.25]  # C4 E4 G4 C5
    note_len = seconds(0.15)
    sig = np.array([], dtype=float)

    for freq in notes:
        t = np.linspace(0, 0.15, note_len)
        tone  = 0.7 * np.sin(2 * np.pi * freq * t)
        tone += 0.2 * np.sin(2 * np.pi * freq * 2 * t)
        tone += 0.1 * np.sin(2 * np.pi * freq * 3 * t)
        env = envelope(note_len, attack=0.01, decay=0.15, sustain=0.5, release=0.3)
        sig = np.concatenate([sig, tone * env])

    write_wav("win.wav", sig)


# ── Entry point ───────────────────────────────────────────────────────────────

def main():
    print("Generating audio assets …")
    make_footstep()
    make_ambient()
    make_interact()
    make_ember_light()
    make_win()
    print("Done.")


main()
