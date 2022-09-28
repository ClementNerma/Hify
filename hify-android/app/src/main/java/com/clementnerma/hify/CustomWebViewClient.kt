package com.clementnerma.hify

import android.net.http.SslError
import android.webkit.*

class CustomWebViewClient(val onError: (errorMsg: String) -> Unit) : WebViewClient() {
    var loaded = false

    private fun isSameUrl(a: String, b: String): Boolean {
        fun cleanupUrl(url: String): String {
            return url
                .replace("(?<!:)/{2,}".toRegex(), "")
                .replace("\\?.*$".toRegex(), "")
                .replace("/$".toRegex(), "")
        }

        return cleanupUrl(a) == cleanupUrl(b)
    }

    override fun onPageCommitVisible(view: WebView?, url: String?) {
        this.loaded = true
        super.onPageCommitVisible(view, url)
    }

    override fun onReceivedHttpError(
        view: WebView?,
        request: WebResourceRequest?,
        errorResponse: WebResourceResponse?
    ) {
        val viewUrl = view?.url
        val requestUrl = request?.url?.toString()

        if (viewUrl != null && requestUrl != null && this.isSameUrl(viewUrl, requestUrl)) {
            this.onError("HTTP error: ${errorResponse?.reasonPhrase ?: "<unspecified>"}")
        }

        super.onReceivedHttpError(view, request, errorResponse)
    }

    override fun onReceivedSslError(view: WebView?, handler: SslErrorHandler?, error: SslError?) {
        this.onError("SSL error")
        super.onReceivedSslError(view, handler, error)
    }

    override fun onReceivedError(
        view: WebView?,
        request: WebResourceRequest?,
        error: WebResourceError?
    ) {
        if (error != null) {
            val message = when (error.errorCode) {
                ERROR_AUTHENTICATION -> "User authentication failed on server"
                ERROR_TIMEOUT -> "The server is taking too much time to communicate. Try again later."
                ERROR_TOO_MANY_REQUESTS -> "Too many requests during this load"
                ERROR_UNKNOWN -> "Generic error"
                ERROR_BAD_URL -> "Check entered URL.."
                ERROR_CONNECT -> "Failed to connect to the server"
                ERROR_FAILED_SSL_HANDSHAKE -> "Failed to perform SSL handshake"
                ERROR_HOST_LOOKUP -> "Server or proxy hostname lookup failed"
                ERROR_PROXY_AUTHENTICATION -> "User authentication failed on proxy"
                ERROR_REDIRECT_LOOP -> "Too many redirects"
                ERROR_UNSUPPORTED_AUTH_SCHEME -> "Unsupported authentication scheme (not basic or digest)"
                ERROR_UNSUPPORTED_SCHEME -> "unsupported scheme"
                ERROR_FILE -> "Generic file error"
                ERROR_FILE_NOT_FOUND -> "File not found"
                ERROR_IO -> "The server failed to communicate. Try again later."
                else -> "<unknown error code>"
            }

            val viewUrl = view?.url
            val requestUrl = request?.url?.toString()

            if (viewUrl != null && requestUrl != null && this.isSameUrl(viewUrl, requestUrl)) {
                this.onError(message)
            }
        } else {
            this.onError("Unspecified network error")
        }

        super.onReceivedError(view, request, error)
    }
}