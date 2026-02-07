
// Service to communicate with the Sarah FS Backend (Gateway)
// Replaces the mock file system logic.

const GATEWAY = "http://localhost:8001/api/fs";

interface FSItem {
    name: string;
    path: string;
    is_dir: boolean;
    size: number;
}

export const fsService = {
    // List files in a directory
    listFiles: async (path: string = "C:\\SarahCore"): Promise<{ items: FSItem[], current_path: string }> => {
        try {
            const res = await fetch(`${GATEWAY}/list`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ path })
            });
            if (!res.ok) throw new Error("FS List Failed");
            return await res.json();
        } catch (e) {
            console.error("FS List Error:", e);
            throw e;
        }
    },

    // Read file content
    readFile: async (path: string): Promise<string> => {
        try {
            const res = await fetch(`${GATEWAY}/read`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ path })
            });
            const data = await res.json();
            if (data.status === "ERROR") throw new Error(data.message);
            return data.content || "";
        } catch (e) {
            console.error("FS Read Error:", e);
            throw e;
        }
    },

    // Write content to file
    writeFile: async (path: string, content: string): Promise<boolean> => {
        try {
            const res = await fetch(`${GATEWAY}/write`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ path, content })
            });
            const data = await res.json();
            return data.status === "SUCCESS";
        } catch (e) {
            console.error("FS Write Error:", e);
            return false;
        }
    }
};
