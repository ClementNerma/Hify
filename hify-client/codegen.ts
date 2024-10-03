import type { CodegenConfig } from '@graphql-codegen/cli'

export default {
	schema: 'http://localhost:8893/graphql',
	documents: ['src/**/*.vue', 'src/**/*.ts', 'src/**/*.gql'],
	generates: {
		'./src/graphql/generated/': {
			preset: 'client',
			presetConfig: {
				fragmentMasking: false,
			},
			config: {
				useTypeImports: true,
				scalars: {
					Empty: {
						input: 'null',
						output: 'null',
					},
				},
			},
		},
	},
} satisfies CodegenConfig
