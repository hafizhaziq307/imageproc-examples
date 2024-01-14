<script>
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { isEmpty } from "../../helper.js";

    let uploadImage;
    let inputImage, outputImage;
    let isProcessing = false;

    async function run() {
        isProcessing = true;

        const res = await invoke("sharpen3x3", { 
            path: uploadImage.path,
        });

        isProcessing = false;

        outputImage.src = `data:image/${uploadImage.extension};base64, ${res.image}`;
    }

    const openDialog = async () => {
        const path = await open({
            multiple: false,
            directory: false,
        });

        if (isEmpty(path)) {
            return;
        }

        uploadImage = {
            path: path,
            extension: path.substr(path.lastIndexOf('.') + 1)
        };

        inputImage.src = convertFileSrc(uploadImage.path);
    };
</script>

<section class="p-4 space-y-2 border border-black rounded bg-white">
    <header class="flex gap-2 items-center">
       <div class="font-bold text-xl capitalize">sharpen 3x3</div>
       <button class="px-3 py-1 rounded bg-blue-600 hover:bg-blue-500 text-white text-xs font-medium" on:click={openDialog}>Upload</button>
       <button class="px-3 py-1 rounded bg-blue-600 hover:bg-blue-500 text-white text-xs font-medium" on:click={run}>Run</button>
    </header>

    <div class="grid grid-cols-2 gap-2">
        <div>
            <img src="" alt="input" class="border" bind:this={inputImage}>
        </div>

        <div>
            <div class="grid place-items-center flex-grow { (isProcessing) ? '' : 'hidden' }">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12 icon icon-tabler icon-tabler-loader-3 animate-spin" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 12a9 9 0 0 0 9 9a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9" /><path d="M17 12a5 5 0 1 0 -5 5" /></svg>
            </div>

            <img src="" alt="output" class="border {(isProcessing) ? 'hidden' : ''}" bind:this={outputImage}>
        </div>
    </div>
</section>