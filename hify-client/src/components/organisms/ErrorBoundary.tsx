import type { ErrorInfo } from 'react'
import React, { Component } from 'react'

type ErrorBoundaryProps = {
  children: React.ReactNode
}

type ErrorBoundaryState = {
  error: Error | null
}

export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props)
    this.state = { error: null }
  }

  static getDerivedStateFromError(error: Error) {
    // Update state so the next render will show the fallback UI.
    return { error }
  }

  // oxlint-disable-next-line class-methods-use-this
  override componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // showNotification({ type: 'error', title: 'React Error', message: error.message })
  }

  override render() {
    if (this.state.error) {
      return (
        <div className="px-5">
          <h2>Something went wrong!</h2>
          <pre>Details: {this.state.error.message}</pre>
          {/*
          <NavItem onPress={() => location.reload()} className="mt-5 p-2 border-gray-600">
            Reload
          </NavItem> */}
        </div>
      )
    }

    return this.props.children
  }
}
