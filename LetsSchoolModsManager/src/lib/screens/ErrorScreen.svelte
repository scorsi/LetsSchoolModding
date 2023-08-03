<script lang="ts">
    import Container from "$lib/components/Container.svelte";

    export let title = "Oups.";
    export let description = ["We are sorry, but something went wrong.", "Please try again later."];
</script>

<Container>
    <div class="flex flex-col gap-6 text-left">
        <h2 class="text-4xl font-bold">{title}</h2>
        <ul>
            {#each description as line}
                <li class="text-lg">{line}</li>
            {/each}
        </ul>
    </div>
</Container>
