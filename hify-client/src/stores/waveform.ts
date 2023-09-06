import { getStreamUri } from '../globals/rest-api'
import { AudioTrackFragment } from '../graphql/generated'
import { logFatal } from './debugger'

export async function fetchAudioBuffer(track: AudioTrackFragment): Promise<ArrayBuffer> {
	const res = await fetch(getStreamUri(track.id))
	return res.arrayBuffer()
}

export async function generateTrackWaveform(from: ArrayBuffer, samples: number): Promise<number[]> {
	const audioCtx = new AudioContext()
	const audioBuffer = await audioCtx.decodeAudioData(from)

	const rawData = audioBuffer.getChannelData(0)

	const blockSize = Math.floor(rawData.length / samples)

	const filteredData = Array.from(Array(samples), (_, i) => {
		const blockStart = blockSize * i

		const sum = rawData.slice(blockStart, blockStart + blockSize).reduce((acc, sample) => acc + Math.abs(sample))

		return sum / blockSize
	})

	// Normalize the data
	const multiplier = Math.max(...filteredData) ** -1
	return filteredData.map((n) => n * multiplier)
}

export function drawComputedTrackWaveForm(canvas: HTMLCanvasElement, normalizedData: number[], progress: number): void {
	canvas.width = canvas.offsetWidth
	canvas.height = canvas.offsetHeight

	const ctx = canvas.getContext('2d')

	if (!ctx) {
		return logFatal('Failed to get 2D drawing context from canvas')
	}

	ctx.clearRect(0, 0, canvas.offsetWidth, canvas.offsetHeight)

	// ctx.scale(window.devicePixelRatio, window.devicePixelRatio)
	// ctx.translate(0, canvas.offsetHeight / 2 + padding) // Set Y = 0 to be in the middle of the canvas

	const middleHeight = canvas.offsetHeight / 2
	const heightRatio = middleHeight / Math.max(...normalizedData)

	ctx.globalAlpha = 0.2
	ctx.fillStyle = '#FFFFFF'
	ctx.fillRect(0, 0, canvas.offsetWidth * progress, canvas.offsetHeight)

	ctx.globalAlpha = 1
	ctx.lineWidth = 1

	ctx.strokeStyle = '#EFEFEF'
	ctx.beginPath()
	ctx.moveTo(0, middleHeight)
	ctx.lineTo(canvas.offsetWidth, middleHeight)
	ctx.stroke()

	const width = canvas.offsetWidth / normalizedData.length

	for (let i = 0; i < normalizedData.length; i++) {
		const x = width * i
		const y = normalizedData[i] * heightRatio

		ctx.strokeStyle = i / normalizedData.length < progress ? '#FF9900' : '#FFFFFF'
		ctx.beginPath()
		ctx.moveTo(x, middleHeight - y)
		ctx.lineTo(x, middleHeight + y)
		ctx.stroke()
	}
}
