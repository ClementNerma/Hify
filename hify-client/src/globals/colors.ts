import { ArtRgb } from '../graphql/generated'

// TODO: optimize this function ^^"
export function lowerBrightness(color: ArtRgb, brightness: number): ArtRgb {
	let roundedColor = color

	while (perceivedBrightness(roundedColor) > brightness) {
		color = multiplyBrightnessRaw(color, 0.95)
		roundedColor = roundColor(color)
	}

	return roundedColor
}

export function multiplyBrightness(color: ArtRgb, coeff: number): ArtRgb {
	return roundColor(multiplyBrightnessRaw(color, coeff))
}

export function multiplyBrightnessRaw(color: ArtRgb, coeff: number): ArtRgb {
	return {
		r: color.r * coeff,
		g: color.g * coeff,
		b: color.b * coeff,
	}
}

export function roundColor(color: ArtRgb): ArtRgb {
	return {
		r: Math.round(color.r),
		g: Math.round(color.g),
		b: Math.round(color.b),
	}
}

// Code below is based on: https://gist.github.com/mnpenner/70ab4f0836bbee548c71947021f93607
// Itself based on:        https://stackoverflow.com/a/56678483/65387

function sRGBtoLin(colorChannel: number) {
	// Send this function a decimal sRGB gamma encoded color value
	// between 0.0 and 1.0, and it returns a linearized value.

	if (colorChannel <= 0.04045) {
		return colorChannel / 12.92
	}

	return Math.pow((colorChannel + 0.055) / 1.055, 2.4)
}

/**
 * @param r Red, [0-1]
 * @param g Green, [0-1]
 * @param b Blue, [0-1]
 * @returns Luminance, [0-1]
 */
function rgbToY(r: number, g: number, b: number) {
	return 0.2126 * sRGBtoLin(r) + 0.7152 * sRGBtoLin(g) + 0.0722 * sRGBtoLin(b)
}

/**
 * Luminance to perceived lightness.
 *
 * @param Y Luminance, [0-1]
 */
function YtoLstar(Y: number) {
	// Send this function a luminance value between 0.0 and 1.0,
	// and it returns L* which is "perceptual lightness"

	if (Y <= 216 / 24389) {
		// The CIE standard states 0.008856 but 216/24389 is the intent for 0.008856451679036
		return Y * (24389 / 27) // The CIE standard states 903.3, but 24389/27 is the intent, making 903.296296296296296
	}

	return Math.pow(Y, 1 / 3) * 116 - 16
}

/**
 * Calculate perceived lightness from RGB hex string.
 *
 * @returns Lightness value, [0-100].
 */
export function perceivedBrightness({ r, g, b }: ArtRgb) {
	return YtoLstar(rgbToY(r / 255, g / 255, b / 255))
}
