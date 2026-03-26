export function LoadingIndicator({ size, top, left }: { size: number; top: number; left: number }) {
  return (
    <div
      className="fixed"
      style={{
        // 'fontSize' controls the overall size of the spinner
        fontSize: size * 2,

        // Center the spinner based on its size (half of the width and height defined below)
        top: `calc(${top}px - (var(--spacing) * 2.5))`,
        left: `calc(${left}px - (var(--spacing) * 2.5))`,
      }}
    >
      {/* <!-- Spinner Container --> */}
      <div className="flex items-center justify-center bg-transparent">
        {/* <!-- macOS Beachball Spinner --> */}
        <div
          className="w-5 h-5 animate-spin rounded-full shadow-lg bg-[conic-gradient(from_0deg,var(--color-blue-500)_0deg_60deg,var(--color-purple-500)_60deg_120deg,var(--color-pink-500)_120deg_180deg,var(--color-orange-500)_180deg_240deg,var(--color-yellow-400)_240deg_300deg,var(--color-green-500)_300deg_360deg)]"
          role="status"
          aria-label="loading"
        >
          {/* <!-- Optional: White Glare Overlay for 3D effect --> */}
          <div className="w-full h-full rounded-full bg-[radial-gradient(circle_at_35%_35%,rgba(255,255,255,0.4),transparent_50%)]" />
        </div>
      </div>
    </div>
  )
}
