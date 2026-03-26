import type { PropsWithChildren } from 'react'

export function FadeWhenUnfocused({ children }: PropsWithChildren) {
  return (
    // TODO: find a way to combine classes under the same Tailwind selector
    <div className="nav-unfocused:opacity-20 nav-unfocused:transition nav-unfocused:ease-linear nav-unfocused:delay-200 nav-unfocused:duration-700">
      {children}
    </div>
  )
}
