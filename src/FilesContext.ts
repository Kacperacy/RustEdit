import React, { createContext, useState, useContext } from "react";
import OpenedFile from "./types/OpenedFile";

const FilesContext = createContext<OpenedFile[]>([]);

type FileProviderProps = {
  children: React.ReactNode;
};

export const FileProvider = ({ children: FileProviderProps }) => {
  const [files, setFiles] = useState<OpenedFile[]>([]);
  return (
    <FilesContext.Provider value={[files, setFiles]}>
      {children}
    </FilesContext.Provider>
  );
};

// Custom hook to use the context
export const useFilesContext = () => useContext(FilesContext);
