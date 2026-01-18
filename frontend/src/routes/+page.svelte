<script module lang="ts">
    import type { Load } from "@sveltejs/kit";

    export const load: Load = async () => {
        throw redirect(307, "/settings");
    };
</script>

<script lang="ts">
    import { redirect } from "@sveltejs/kit";
</script>
