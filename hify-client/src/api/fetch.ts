import { ArkErrors, type } from 'arktype'
import { showFailure } from '#/global/notifications.ts'
import { tryFallible, tryFallibleAsync, type JsonStringifyable } from '#/utils/common.ts'

export const API_DOMAIN = `http://${location.hostname}:8891`

const apiResponse = type([{ ok: 'true', data: 'unknown' }, '|', { ok: 'false', cause: 'string' }])

async function callRemoteApi<Validator extends type.Any>(
  method: 'GET' | 'POST' | 'PUT' | 'DELETE',
  uri: string,
  queryParams: Record<string, JsonStringifyable> | null,
  bodyParams: Record<string, JsonStringifyable> | null,
  validator: Validator,
): Promise<Validator['inferOut']> {
  const url = new URL(
    `${API_DOMAIN}${API_DOMAIN.endsWith('/') || uri.startsWith('/') ? '' : '/'}${uri}`,
  )

  for (const [name, value] of Object.entries(queryParams ?? {})) {
    if (value !== null) {
      url.searchParams.set(name, typeof value === 'string' ? value : JSON.stringify(value))
    }
  }

  const req: RequestInit = { method }

  if (bodyParams) {
    req.headers = { 'Content-Type': 'application/json' }
    req.body = JSON.stringify(bodyParams)
  }

  console.debug(
    `API call: ${method} ${uri}${url.searchParams.size > 0 ? `?${url.searchParams.toString()}` : ''}`,
  )

  const res = await tryFallibleAsync(() => fetch(url, req))

  if (res instanceof Error) {
    throw new TypeError(`Failed to fetch "${uri}": ${res.message}`)
  }

  const text = await res.text()

  if (!res.ok) {
    const jsonError = tryFallible(() => JSON.parse(text) as unknown)
    const parsedError = apiResponse.onDeepUndeclaredKey('reject')(jsonError)

    if (!(parsedError instanceof ArkErrors)) {
      if (!parsedError.ok) {
        throw new Error(`Request failed for "${uri}":\n\n${parsedError.cause}`)
      }

      showFailure('Unexpected "ok" status in error response from the API')
    }

    throw new Error(`Request failed for "${uri}" (status: ${res.status}):\n\n${text}`)
  }

  if (res.status !== 200) {
    throw new Error(`Non-200 status code for "${uri}" (${res.status}) : ${text}`)
  }

  const json = tryFallible(() => JSON.parse(text) as unknown)

  if (json instanceof Error) {
    throw new TypeError(
      `Failed to parse the fetched response for "${uri}" as JSON: ${json.message}`,
    )
  }

  const parsed = apiResponse.onDeepUndeclaredKey('reject')(json)

  if (parsed instanceof ArkErrors) {
    throw new TypeError(
      `The API response for "${uri}" does not follow the expected JSON structure: ${parsed.summary}`,
    )
  }

  if (!parsed.ok) {
    throw new Error(`API returned an error: ${parsed.cause}`)
  }

  const out: unknown = validator.onDeepUndeclaredKey('reject')(parsed.data)

  if (out instanceof ArkErrors) {
    throw new TypeError(
      `API's successful response for "${uri}" does not have the expected JSON structure: ${out.summary}`,
    )
  }

  return out
}

//
// => Utils
//

export function queryApi<Validator extends type.Any>(
  uri: string,
  queryParams: Record<string, JsonStringifyable> | null,
  validator: Validator,
  bodyParams?: Record<string, JsonStringifyable>,
): Promise<Validator['inferOut']> {
  return callRemoteApi('GET', uri, queryParams, bodyParams ?? null, validator)
}

export async function callApiMutation(
  method: 'POST' | 'PUT' | 'DELETE',
  uri: string,
  bodyParams?: Record<string, JsonStringifyable>,
): Promise<void> {
  await callRemoteApi(method, uri, null, bodyParams ?? null, type('null'))
}
