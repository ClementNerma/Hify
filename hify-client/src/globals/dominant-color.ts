export async function computeDominantColor(imgUrl: string): Promise<[r: number, g: number, b: number]> {
  const img = new Image()
  img.crossOrigin = 'Anonymous'
  img.src = imgUrl

  await new Promise((resolve) => img.addEventListener('load', resolve))

  if (!img.width || !img.height) {
    throw new Error('Undefined image width and/or height')
  }

  const canvas = document.createElement('canvas')
  canvas.width = img.width
  canvas.height = img.height

  const ctx = canvas.getContext('2d')

  if (!ctx) {
    throw new Error('Failed to get 2D drawing context from temporary canvas')
  }

  ctx.drawImage(img, 0, 0)

  const pixels = ctx.getImageData(0, 0, canvas.width, canvas.height).data
  const pixelsCount = canvas.width * canvas.height
  console.log(pixelsCount)

  if (pixels.length !== pixelsCount * 4) {
    throw new Error('Invalid image data length from temporary canvas')
  }

  const sum = { r: 0, g: 0, b: 0 }

  for (let p = 0; p < pixelsCount; p++) {
    sum.r += pixels[p * 4]
    sum.g += pixels[p * 4 + 1]
    sum.b += pixels[p * 4 + 2]
  }

  const mean = (sum: number) => Math.floor(sum / pixelsCount)

  return [mean(sum.r), mean(sum.g), mean(sum.b)]
}
