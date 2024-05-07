<script lang='ts'>
    import { invoke } from '@tauri-apps/api/tauri';
    import { dialog, fs } from '@tauri-apps/api';
    import Papa from 'papaparse';
    import Icon from '@iconify/svelte';

    let height: number;
    let width: number;
    $: height = window.innerHeight;
    $: width = window.innerWidth;

    let missingFiles: String[] = [];
    let base_path: string = "";

    async function checkFilesExistence(filePaths: String[]) {
        for (let path of filePaths) {
            await invoke('check_file_exists', { filePath: path }).then((result) => {
                if (!result) {
                    if (path.includes('VMG')) {
                        let newPath = path.replace("01. Zagen", "02. Frezen");
                        invoke('check_file_exists', { filePath: newPath }).then((result) => {
                            !result && addToMissingFiles(newPath);
                        })
                    } else {
                        addToMissingFiles(path);
                    }
                }
            })
        }
    }

    function addToMissingFiles(path: String) {
        if (!missingFiles.includes(path)) {
            missingFiles = [...missingFiles, path];
        }
    }

    async function readCSVfile(filePath: String) {
        try {
            const fileContent = await fs.readTextFile(filePath.toString());
            const results = Papa.parse(fileContent, {
                header: true,
                skipEmptyLines: true
            });

            if (!results.data || !Array.isArray(results.data)) {
                console.error("Error in de CSV data");
                return [];
            }
            switch(true) {
                case filePath.includes('VH'):
                    return createVHPath(results.data);
                case filePath.includes('BB'):
                    return createBBPath(results.data);
                case filePath.includes('VMG'):
                    return createVMGPath(results.data);
                default:
                    return [];
            }
        } catch (error) {
            console.error(`Error bij het lezen of verwerken van ${filePath}:`, error);
            return [];
        }
    }

    function createVHPath(dataRows: any[]) {
        let paths: any = [];
        for (let row of dataRows) {
            let path = "01. VH\\" + row['Bestand'].split("\\").slice(3).join("\\").split('.')[0];
            paths.push(path);
        }
        return paths;
    }

    function createBBPath(dataRows: any[]) {
        let paths: any = [];
        for (let row of dataRows) {
            let path = "02. BB\\" + row['OnderdeelNaam'].split(' ')[0] + " " + row['OnderdeelNaam'].split(' ')[1] + "\\" + row['Productcode']
            paths.push(path);
        }
        return paths;
    }
    
    function createVMGPath(dataRows: any[]) {
        let paths: any = [];
        for (let row of dataRows) {
            let path = "03. VMG\\01. Zagen\\" + row['Productcode'];
            paths.push(path);
        }
        return paths;
    }

    async function readCSVfiles(filePaths: String[]) {
        let file_paths: String[] = [];
        for (let filePath of filePaths) {
            if (filePath.includes('.csv')) {
                const values = await readCSVfile(filePath);
                file_paths = [...file_paths, ...values];
            }
        }
        for (let file in file_paths) {
            file_paths[file] = base_path + "\\" + file_paths[file];
        }
        const unique = [...new Set(file_paths)];
        await checkFilesExistence(unique);
    }

    async function openLocation(filePath: String) {
        let directory = filePath.substring(0, filePath.lastIndexOf('\\'));
        await invoke('show_in_folder', { path: directory });
    }

    async function setBasePath() {
        const result = await dialog.open({
            directory: true
        });
        if (result) {
            base_path = result.toString();
        }
    }

    async function setCSVPath() {
        const result = await dialog.open({
            multiple: true,
            filters: [{ name: 'CSV', extensions: ['csv'] }]
        }) as string[];
        if (result) {
            missingFiles = [];
            readCSVfiles(result.map((entry) => entry));
        }
    }

</script>

<svelte:window bind:innerHeight={height} bind:innerWidth={width} />
<header>
    <h1 class='header-title'>Checker voor de DWG bestanden</h1>
</header>
<main style="height: calc({height}px - 11rem); width: {width}px;" class='shape-divider'>
    <button class='button' on:click={setBasePath}><span>Pad voor Autodesk</span></button>
    {#if base_path !== ""}
        <p class='pad'>Pad van 02. CNC: {base_path}</p>
    {:else}
        <p class='pad'>Selecteer het pad voor 02. CNC</p>
    {/if}
    {#if base_path !== ""}
        <button class='button' on:click={setCSVPath}><span>Folder met CSVs</span></button>
        <ul>
            {#each missingFiles as file}
                <li>
                    <span>{file}</span>
                    <button on:click={() => openLocation(file)}><Icon icon="material-symbols:folder-open"/></button>
                </li>
            {/each}
        </ul>
    {/if}
</main>