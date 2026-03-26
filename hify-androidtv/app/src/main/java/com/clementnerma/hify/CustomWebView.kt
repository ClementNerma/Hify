package com.clementnerma.hify

import android.annotation.SuppressLint
import android.content.Context
import android.util.AttributeSet
import android.util.Log
import android.view.KeyEvent
import android.webkit.WebSettings
import android.webkit.WebView
import android.widget.Toast

class CustomWebView(context: Context, attrs: AttributeSet) : WebView(context, attrs) {
    init {
        this.initView()
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun initView() {
        this.settings.javaScriptEnabled = true
        this.settings.mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
        this.settings.allowContentAccess = true
        this.settings.domStorageEnabled = true
        this.settings.mediaPlaybackRequiresUserGesture = false

        this.settings.loadWithOverviewMode = true
        this.settings.useWideViewPort = true
    }

    // Key presses are intercepted to prevent arrows from putting the browser's (native) focus
    // unwantedly on items, which causes lots of problems with the arrow-based navigation manager
    // in the client.
    override fun dispatchKeyEvent(event: KeyEvent?): Boolean {
        var finalEvent = event

        // List of keys to catch and their translation for the JS event catching
        val translatedKeys = mapOf(
            KeyEvent.KEYCODE_BACK to KeyEvent.KEYCODE_F1, // triggered in .handleBackButton() below
            KeyEvent.KEYCODE_DPAD_CENTER to KeyEvent.KEYCODE_F2,

            KeyEvent.KEYCODE_DPAD_UP to KeyEvent.KEYCODE_F3,
            KeyEvent.KEYCODE_DPAD_DOWN to KeyEvent.KEYCODE_F4,

            // Don't bind to F5 (page refresh)

            KeyEvent.KEYCODE_DPAD_LEFT to KeyEvent.KEYCODE_F6,
            KeyEvent.KEYCODE_DPAD_RIGHT to KeyEvent.KEYCODE_F7,
        )

        // Check if this is a key event (though dispatchEvent can handle others)
        if (event != null &&
            !event.isMetaPressed &&
            !event.isCtrlPressed &&
            !event.isAltPressed &&
            !event.isShiftPressed &&
            translatedKeys.containsKey(event.keyCode))
        {
            val newKeyCode = translatedKeys.getValue(event.keyCode)

            // Create a new KeyEvent with the same properties but modified keyCode
            finalEvent = KeyEvent(
                event.downTime,  // Same down time
                event.eventTime, // Same event time
                event.action,    // Same action (e.g., ACTION_DOWN, ACTION_UP)
                newKeyCode,      // Modified keyCode
                event.repeatCount, // Same repeat count
                event.metaState,  // Same meta state
                event.deviceId,   // Same device ID
                event.scanCode,   // Same scan code
                event.flags,      // Same flags
                event.source      // Same source
            );
        }

        return super.dispatchKeyEvent(finalEvent)
    }

    // This method is called from `MainActivity`'s `onBackPressed` handler, which has no equivalent
    // inside a `WebView`
    fun handleBackButton() {
        this.dispatchKeyEvent(KeyEvent(KeyEvent.ACTION_DOWN, KeyEvent.KEYCODE_BACK))
        this.dispatchKeyEvent(KeyEvent(KeyEvent.ACTION_UP, KeyEvent.KEYCODE_BACK))
    }
}