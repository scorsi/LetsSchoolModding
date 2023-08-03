<script lang="ts">
    import {createEventDispatcher, onMount} from "svelte";
    import LoadingScreen from "$lib/screens/LoadingScreen.svelte";
    import {modLoader} from "$lib/commands";
    import ErrorScreen from "$lib/screens/ErrorScreen.svelte";
    import Container from "$lib/components/Container.svelte";
    import {ProgressBar} from '@skeletonlabs/skeleton';
    import store from "$lib/store";

    const dispatch = createEventDispatcher();

    let isError = false;
    let isLoading = true;
    let isModLoaderInstalled = false;
    let isModLoaderOutdated = false;

    let isDownloading = false;
    let isInstalling = false;
    let isCleaning = false;

    function validateModLoader() {
        isLoading = true;
        setTimeout(() => {
            modLoader.check()
                .then((result) => {
                    isModLoaderInstalled = result !== "not-installed";
                    isModLoaderOutdated = result === "outdated";

                    if (isModLoaderInstalled && !isModLoaderOutdated) {
                        isLoading = false;
                        store.update((s) => ({ ...s, modLoaderIsInstalled: true }));
                        setTimeout(() => {
                            dispatch("modLoaderInstalled");
                        }, 1500);
                    } else if (isModLoaderInstalled && isModLoaderOutdated) {
                        setTimeout(() => {
                            isLoading = false;
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

    function installModLoader() {
        isDownloading = true;
        store.update((s) => ({ ...s, modLoaderIsInstalling: true }));
        setTimeout(() => {
            modLoader.download()
                .then(() => {
                    isDownloading = false;
                    isInstalling = true;
                    setTimeout(() => {
                        modLoader.install()
                            .then(() => {
                                isInstalling = false;
                                isCleaning = true;
                                setTimeout(() => {
                                    modLoader.cleanDownload()
                                        .then(() => {
                                            isCleaning = false;
                                            validateModLoader();
                                        })
                                        .catch((err) => {
                                            console.error(err);
                                            isError = true;
                                            setTimeout(() => {
                                                isLoading = false;
                                                isError = false;
                                            }, 1500);
                                        });
                                }, 1500);
                            })
                            .catch((err) => {
                                console.error(err);
                                isError = true;
                                setTimeout(() => {
                                    isLoading = false;
                                    isError = false;
                                }, 1500);
                            });
                    }, 1500);
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

    onMount(validateModLoader);
</script>

{#if isLoading && isError}
    <Container>
        <div class="flex flex-col gap-0.5">
            <ProgressBar value={100} meter="bg-error-400-500-token"/>
            <p class="text-left text-error-400-500-token">Mod loader not installed.</p>
        </div>
    </Container>
{:else if isDownloading}
    <LoadingScreen text="Downloading mod loader..."/>
{:else if isInstalling}
    <LoadingScreen text="Installing mod loader..."/>
{:else if isCleaning}
    <LoadingScreen text="Cleaning up..."/>
{:else if isError}
    <ErrorScreen/>
{:else if isLoading && isModLoaderOutdated}
    <Container>
        <div class="flex flex-col gap-0.5">
            <ProgressBar value={100} meter="bg-warning-400-500-token"/>
            <p class="text-left text-warning-400-500-token">New update available for mod loader.</p>
        </div>
    </Container>
{:else if isLoading}
    <LoadingScreen text="Checking mod loader installation..."/>
{:else if !isModLoaderInstalled}
    <Container>
        <div class="flex flex-col gap-6">
            <h2 class="text-2xl">Mod loader not installed, install ?</h2>
            <div class="flex flex-row gap-6 px-16">
                <button class="flex-1 btn btn-sm variant-filled-primary" on:click={installModLoader}>Install</button>
            </div>
        </div>
    </Container>
{:else if isModLoaderOutdated}
    <Container>
        <div class="flex flex-col gap-6">
            <h2 class="text-2xl">Mod loader outdated, update ?</h2>
            <div class="flex flex-row gap-6 px-16">
                <button class="flex-1 btn btn-sm variant-filled-primary">Update</button>
                <button class="flex-1 btn btn-sm variant-filled-warning" on:click={() => dispatch("modLoaderInstalled")}>Ignore</button>
            </div>
        </div>
    </Container>
{:else if isModLoaderInstalled}
    <Container>
        <div class="flex flex-col gap-0.5">
            <ProgressBar value={100} meter="bg-success-400-500-token"/>
            <p class="text-left text-success-400-500-token">Mod loader installed and up to date.</p>
        </div>
    </Container>
{/if}
