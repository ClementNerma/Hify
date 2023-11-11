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
		showErrorDialog('GraphQL - Network Error', networkError.message)
	}

	if (graphQLErrors) {
		showErrorDialog(
			'GraphQL - API Error',
			`Encountered the following error(s): ${graphQLErrors.map((err) => `* ${err.message}`).join('\n')}`,
		)
	}
})

export default new ApolloClient({
	link: from([
		errorHandler,

		new HttpLink({ uri: `${API_SERVER_URL}/graphql` }),
	]),

	cache: new InMemoryCache(),

	// Disable caching entirely
	// This is HIGHGLY discouraged normally, but is actually relevant here
	// as we don't have a good way to make children components trigger
	// a data refetch from their parent
	defaultOptions: {
		query: {
			fetchPolicy: 'no-cache',
		},
		watchQuery: {
			fetchPolicy: 'no-cache',
		},
		mutate: {
			fetchPolicy: 'no-cache',
		},
	},
})
