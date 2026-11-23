/*
 * Copyright 2026 Corrode contributors
 * SPDX-License-Identifier: GPL-2.0-only
 */
package org.telegram.messenger;

import androidx.annotation.Keep;

/** Narrow JNI boundary for the first Corrode Rust migration slice. */
@Keep
public final class CorrodeNative {
    static {
        System.loadLibrary("corrode_native");
    }

    private CorrodeNative() {
    }

    /** Returns ASCII digits, or null when Java should use its existing fallback. */
    public static String normalizePhone(String rawPhone) {
        return nativeNormalizePhone(rawPhone);
    }

    private static native String nativeNormalizePhone(String rawPhone);
}
