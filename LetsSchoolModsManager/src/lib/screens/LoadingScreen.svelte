<script lang="ts">
    import {ProgressBar} from "@skeletonlabs/skeleton";
    import Container from "$lib/components/Container.svelte";

    export let text = "Loading...";
</script>

<Container>
    <div class="flex flex-col gap-0.5">
        <ProgressBar />
        <p class="text-left">{text}</p>
    </div>
</Container>
