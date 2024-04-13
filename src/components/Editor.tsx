import Editor from "@monaco-editor/react";

const CodeEditor: React.FC = () => {
  return (
    <Editor
      height="100%"
      defaultLanguage="rust"
      defaultValue="// Write your Rust code here"
      theme="vs-dark"
    />
  );
};

export default CodeEditor;
