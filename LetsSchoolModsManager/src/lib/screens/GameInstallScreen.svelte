<script lang="ts">
    import {createEventDispatcher, onMount} from "svelte";
    import LoadingScreen from "$lib/screens/LoadingScreen.svelte";
    import {game} from "$lib/commands";
    import ErrorScreen from "$lib/screens/ErrorScreen.svelte";
    import store from "$lib/store";
    import Container from "$lib/components/Container.svelte";
    import type {PopupSettings} from "@skeletonlabs/skeleton";
    import {popup, ProgressBar} from '@skeletonlabs/skeleton';
    import {DEFAULT_GAME_INSTALLATION_FOLDER} from "$lib/constants";

    const dispatch = createEventDispatcher();

    let isError = false;
    let isLoading = true;
    let isGameInstalled = false;

    let gamePath = DEFAULT_GAME_INSTALLATION_FOLDER;

    function validateGamePath() {
        isLoading = true;
        setTimeout(() => {
            game.check(gamePath)
                .then((result) => {
                    isGameInstalled = !!result;

                    if (isGameInstalled) {
                        isLoading = false;
                        store.update((s) => ({...s, gameInstallationFolder: gamePath}));
                        setTimeout(() => {
                            dispatch("gameInstalled");
                        }, 1500);
                    } else {
                        isError = true;
                        setTimeout(() => {
                            isLoading = false;
                            isError = false;
                        }, 1500);
                    }
                })
                .catch((err) => {
                    console.error(err);
                    isError = true;
                    setTimeout(() => {
                        isLoading = false;
                        isError = false;
                    }, 1500);
                });
        }, 2000);
    }

    onMount(validateGamePath);

    const popupHover: PopupSettings = {
        event: 'hover',
        target: 'whereismygame',
        placement: 'bottom'
    };
</script>

{#if isLoading && isError}
    <Container>
        <div class="flex flex-col gap-0.5">
            <ProgressBar value={100} meter="bg-error-400-500-token" />
            <p class="text-left text-error-400-500-token">Game not found.</p>
        </div>
    </Container>
{:else if isError}
    <ErrorScreen/>
{:else if isLoading}
    <LoadingScreen text="Checking game installation..."/>
{:else if !isGameInstalled}
    <Container>
        <div class="flex flex-col gap-6">
            <div class="flex flex-row gap-3">
                <input class="input" bind:value={gamePath} placeholder="Type the installation folder of your game..."/>
                <button class="btn variant-filled" class:variant-filled-primary={!!gamePath} disabled={!gamePath} on:click={validateGamePath}>Validate</button>
            </div>
            <div>
                <span class="chip variant-outline-primary" use:popup={popupHover}>Where is my Game ?</span>
            </div>
        </div>
    </Container>
{:else}
    <Container>
        <div class="flex flex-col gap-0.5">
            <ProgressBar value={100} meter="bg-success-400-500-token" />
            <p class="text-left text-success-400-500-token">Game found.</p>
        </div>
    </Container>
{/if}

<div class="card p-4 variant-ghost-primary" data-popup="whereismygame">
    <div class="text-xs max-w-lg">
        <p>
            If you don't know where your game is installed:
            right-click on the game in your Steam library,
            select "Properties",
            then "Local Files",
            and finally "Browse Local Files".
        </p>
    </div>
</div>