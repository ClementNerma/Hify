export function bind<T, U>(data: T, callback: (value: T) => U): () => U {
  return () => callback(data)
}
