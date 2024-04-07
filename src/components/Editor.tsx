import { ChangeEvent, useState } from "react";

function Editor() {
  const [code, setCode] = useState<string>("// Start coding here...");

  const handleChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    setCode(event.target.value);
  };

  return (
    <div className="code-editor">
      <textarea
        value={code}
        onChange={handleChange}
        className="code-editor-textarea"
        placeholder="// Start coding here..."
      />
    </div>
  );
}

export default Editor;
