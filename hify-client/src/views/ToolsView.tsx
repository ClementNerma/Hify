import { useApiMutation } from '#/api/hooks.ts'
import { updateIndex } from '#/api/mutations.ts'
import { Button } from '#/components/atoms/Button.tsx'
import { MutationStatus } from '#/components/atoms/MutationIndicator.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'

export function ToolsView() {
  return (
    <div className="px-5">
      <h2>Tools</h2>

      <NavRow>
        <IndexUpdateButton />
      </NavRow>
    </div>
  )
}

function IndexUpdateButton() {
  const mutation = useApiMutation(updateIndex)

  return (
    <Button onPress={() => mutation.run()} disabled={mutation.status === 'pending'}>
      Update index <MutationStatus status={mutation.status} />
    </Button>
  )
}
