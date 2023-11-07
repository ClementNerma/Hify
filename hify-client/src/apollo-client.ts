import { ApolloClient, HttpLink, InMemoryCache, from } from '@apollo/client/core'
import { loadDevMessages, loadErrorMessages } from '@apollo/client/dev'
import { onError } from '@apollo/client/link/error'
import { showErrorDialog } from './components/molecules/ErrorDialog/ErrorDialog'

// Enable logging
loadErrorMessages()
loadDevMessages()

export const API_SERVER_URL = `http://${location.hostname}:8893`

const errorHandler = onError(({ graphQLErrors, networkError }) => {
	if (networkError) {
		showErrorDialog({
			title: 'GraphQL - Network Error',
			message: networkError.message,
			details: null,
		})
	}

	if (graphQLErrors) {
		showErrorDialog({
			title: 'GraphQL - API Error',
			message: `Encountered the following error(s): ${graphQLErrors.map((err) => `* ${err.message}`).join('\n')}`,
			details: null,
		})
	}
})

export default new ApolloClient({
	link: from([
		errorHandler,

		new HttpLink({ uri: `${API_SERVER_URL}/graphql` }),
	]),

	cache: new InMemoryCache({
		resultCaching: false,
	}),
})
