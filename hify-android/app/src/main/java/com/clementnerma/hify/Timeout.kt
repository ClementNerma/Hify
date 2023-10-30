package com.clementnerma.hify

import android.os.CountDownTimer

class Timeout(private val ms: Long, val onTimeout: () -> Unit): CountDownTimer(ms, ms) {
    override fun onTick(p0: Long) {}

    override fun onFinish() {
        this.onTimeout()
    }

}