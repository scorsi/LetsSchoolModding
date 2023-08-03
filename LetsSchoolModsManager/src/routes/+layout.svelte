<script lang='ts'>
    import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
    import '@skeletonlabs/skeleton/styles/skeleton.css';
    import '../app.postcss';

    import {computePosition, autoUpdate, offset, shift, flip, arrow} from '@floating-ui/dom';
    import {storePopup, LightSwitch} from "@skeletonlabs/skeleton";
    import InstallStatus from "$lib/components/InstallStatus.svelte";
    import store from "$lib/store";

    storePopup.set({computePosition, autoUpdate, offset, shift, flip, arrow});
</script>

<div class="p-6 pt-3">
    <h1 class="text-3xl font-extrabold flex flex-row items-start">
        Let's School - Mods Manager
        <InstallStatus isInstalled={$store.modLoaderIsInstalled} isInstalling={$store.modLoaderIsInstalling} />
    </h1>

    <div class="mt-10">
        <slot/>
    </div>
</div>

<div class="absolute bg-surface-800-100-token bottom-0 w-screen p-2 flex justify-between items-center text-surface-50-900-token">
    <div class="flex gap-3 items-center">
        <p class="font-bold text-sm">
            <a href="https://discord.gg/GxXZvqjyFt" target="_blank" class="underline">Join our official Let's School
                Modding Discord !</a>
        </p>
    </div>
    <LightSwitch/>
</div>