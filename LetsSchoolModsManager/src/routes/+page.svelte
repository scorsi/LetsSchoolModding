<script lang="ts">
    import {onMount} from "svelte";
    import LoadingScreen from "$lib/screens/LoadingScreen.svelte";
    import GameInstallScreen from "$lib/screens/GameInstallScreen.svelte";
    import ModLoaderScreen from "$lib/screens/ModLoaderScreen.svelte";
    import ErrorScreen from "$lib/screens/ErrorScreen.svelte";
    import Container from "$lib/components/Container.svelte";

    let step = "loading";

    onMount(() => {
        setTimeout(next, 2000);
    });

    function next() {
        switch (step) {
            case "loading":
                step = "game";
                break;
            case "game":
                step = "modloader";
                break;
            case "modloader":
                step = "done";
                break;
        }
    }
</script>

{#if step === "loading"}
    <LoadingScreen text="Initialization..." />
{:else if step === "game"}
    <GameInstallScreen on:gameInstalled={next} />
{:else if step === "modloader"}
    <ModLoaderScreen on:modLoaderInstalled={next} />
{:else if step === "done"}
    <Container>
        Done
    </Container>
{:else}
    <ErrorScreen />
{/if}
