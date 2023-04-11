import { ApolloClient, InMemoryCache } from '@apollo/client/core'

export const API_SERVER_URL = `http://${location.hostname}:8894`

export default new ApolloClient({
	uri: `${API_SERVER_URL}/graphql`,
	cache: new InMemoryCache(),
})
