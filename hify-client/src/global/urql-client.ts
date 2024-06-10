import { Client, fetchExchange } from '@urql/vue'
import { API_SERVER_URL } from './constants'

export const gqlClient = new Client({
	url: `${API_SERVER_URL}/graphql`,
	exchanges: [fetchExchange],
})
