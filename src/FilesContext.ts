import { createContext } from "react";
import OpenedFile from "./types/OpenedFile";

const FilesContext = createContext<{
  files: OpenedFile[];
  setFiles: React.Dispatch<React.SetStateAction<OpenedFile[]>>;
}>({
  files: [],
  setFiles: () => console.error("FilesContext not initialized"),
});

export default FilesContext;
