import React, { useState, useEffect, ChangeEvent } from "react";

const CodeEditor: React.FC = () => {
  const [code, setCode] = useState<string>("// Start coding here...");

  const handleChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    setCode(event.target.value);
  };

  return (
    <div className="code-editor">
      <textarea
        id="code-textarea"
        value={code}
        onChange={handleChange}
        className="code-editor-textarea"
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
        spellCheck={false}
      />
    </div>
  );
};

export default CodeEditor;
