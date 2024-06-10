<script lang="ts">
import { h, Comment, defineComponent } from 'vue'

export default defineComponent({
    props: {
        content: String
    },

    render() {
        return h(Comment, this.$props.content)
    }
})
</script>
