import { Color } from '../stores/custom-bg-color'
import { logDebug } from '../stores/debugger'
import kmeans from './kmeans'

export async function computeDominantColor(imgUrl: string): Promise<Color> {
  const img = new Image()
  img.crossOrigin = 'Anonymous'
  img.src = imgUrl

  await new Promise((resolve) => img.addEventListener('load', resolve))

  if (!img.width || !img.height) {
    throw new Error('Undefined image width and/or height')
  }

  const width = Math.min(img.width, 100)
  const height = (img.height / img.width) * width

  const canvas = document.createElement('canvas')
  canvas.width = width
  canvas.height = height

  const ctx = canvas.getContext('2d')

  if (!ctx) {
    throw new Error('Failed to get 2D drawing context from temporary canvas')
  }

  ctx.drawImage(img, 0, 0)

  const imgData = ctx.getImageData(0, 0, canvas.width, canvas.height).data
  const pixelsCount = canvas.width * canvas.height

  if (imgData.length !== pixelsCount * 4) {
    throw new Error('Invalid image data length from temporary canvas')
  }

  const pixels = new Array(pixelsCount)
    .fill(null)
    .map((_, i) => [imgData[i * 4], imgData[i * 4 + 1], imgData[i * 4 + 2]])

  const started = Date.now()

  const { centroids } = kmeans(pixels, 5)

  logDebug(`Computed centroids K-Means in ${Date.now() - started} ms.`)

  const [r, g, b] = centroids[0]

  return [Math.round(r), Math.round(g), Math.round(b), 0.5]
}
