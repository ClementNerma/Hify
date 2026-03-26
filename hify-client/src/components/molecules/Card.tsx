export type CardProps = {
  artUrl: string
  title: string
}

export function Card({ artUrl, title }: CardProps) {
  return (
    <div>
      <img className="inline-block w-full" src={artUrl} />

      <div className="flex items-center justify-center min-h-[2.25lh] leading-6 text-center">
        <p className="line-clamp-2 text-center">{title}</p>
      </div>
    </div>
  )
}
