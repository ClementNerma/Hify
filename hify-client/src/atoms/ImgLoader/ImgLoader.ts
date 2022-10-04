import { decode } from 'blurhash'
import { ProgressiveImgFragment } from '../../graphql/generated'

export function createBlurHashImageSrc(art: ProgressiveImgFragment): string {
  const canvas = document.createElement('canvas')
  canvas.width = art.width
  canvas.height = art.height

  const ctx = canvas.getContext('2d')

  if (!ctx) {
    throw new Error('Failed to get 2D drawing context from temporary canvas')
  }

  const decoded = decode(art.blurhash, art.width, art.height)
  const imgData = new ImageData(decoded, art.width, art.height)

  ctx.putImageData(imgData, 0, 0)

  const url = canvas.toDataURL()

  return url
}

// export function createProgressiveImage(art: ProgressiveImgFragment, width: number, height: number): Readable<string> {
//   const imgSrc = writable(createBlurHashImageSrc(art, width, height))

//   const fullImgUrl = getArtUri(art.id)

//   const img = new Image()
//   img.src = fullImgUrl
//   img.addEventListener('load', () => imgSrc.set(fullImgUrl))

//   return readonly(imgSrc)
// }
