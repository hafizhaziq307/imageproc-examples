<script>
    import BilateralFilter from "./lib/filter/BilateralFilter.svelte";
    import MedianFilter from "./lib/filter/MedianFilter.svelte";
    import GaussianBlur from "./lib/filter/GaussianBlur.svelte";
    import Sharpen3x3 from "./lib/filter/Sharpen3x3.svelte";
    import SharpenGaussian from "./lib/filter/SharpenGaussian.svelte";
    import StretchContrast from "./lib/contrast/StretchContrast.svelte";
    import Threshold from "./lib/contrast/Threshold.svelte";
    import Dilate from "./lib/morphology/Dilate.svelte";
    import DistanceTransform from "./lib/distance_transform/DistanceTransform.svelte";
    import Canny from "./lib/edges/Canny.svelte";
    import GaussianNoise from "./lib/noise/GaussianNoise.svelte";
    import SaltAndPepperNoise from "./lib/noise/SaltAndPepperNoise.svelte";
</script>

<main class="p-4 space-y-4">
    <!-- <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Noise</summary>

        <div class="mt-2 space-y-2">
             
        </div>
    </details> -->

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Filter</summary>

        <div class="mt-2 space-y-2">
            <BilateralFilter/>
            <MedianFilter/>
            <GaussianBlur/>
            <Sharpen3x3/>
            <SharpenGaussian/>
        </div>
    </details>

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Contrast</summary>

        <div class="mt-2 space-y-2">
            <StretchContrast/>
            <Threshold/>
        </div>
    </details>

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Morphology</summary>

        <div class="mt-2 space-y-2">
            <Dilate/>
        </div>
    </details>

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Distance Transform</summary>

        <div class="mt-2 space-y-2">
            <DistanceTransform/> 
        </div>
    </details>

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Edges</summary>

        <div class="mt-2 space-y-2">
            <Canny/>
        </div>
    </details>

    <details>
        <summary class="text-2xl font-medium underline underline-offset-4 cursor-pointer">Noise</summary>

        <div class="mt-2 space-y-2">
            <GaussianNoise/>
            <SaltAndPepperNoise/>
        </div>
    </details>

</main>