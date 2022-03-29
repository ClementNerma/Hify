import { ApolloClient, InMemoryCache } from '@apollo/client/core'

export const API_SERVER_URL = 'http://localhost:8000/graphql'

export default new ApolloClient({
  uri: API_SERVER_URL,
  cache: new InMemoryCache(),
})
