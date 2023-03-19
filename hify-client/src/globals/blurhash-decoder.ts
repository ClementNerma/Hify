import { decode } from 'blurhash-wasm'
import { ProgressiveImgFragment } from "../graphql/generated"

export function createBlurHashImageSrc(art: ProgressiveImgFragment, width: number, height: number): string {
  // const width = PLACEHOLDER_WIDTH
  // const height = Math.round((art.height / art.width) * PLACEHOLDER_WIDTH)

  const canvas = document.createElement('canvas')
  canvas.width = width
  canvas.height = height

  const ctx = canvas.getContext('2d')

  if (!ctx) {
    throw new Error('Failed to get 2D drawing context from temporary canvas')
  }

  const decoded = decode(art.blurhash, width, height)

  if (!decoded) {
    throw new Error('Failed to decode blur hash :(')
  }

  const imgData = new ImageData(new Uint8ClampedArray(decoded), width, height)

  ctx.putImageData(imgData, 0, 0)

  return canvas.toDataURL()
}