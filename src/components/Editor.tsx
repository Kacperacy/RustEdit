import React, { useState, useEffect, ChangeEvent, MouseEvent } from "react";

const CodeEditor: React.FC = () => {
  const [code, setCode] = useState<string>("// Start coding here...");
  const [lineCount, setLineCount] = useState<number>(1);
  const [selectedLine, setSelectedLine] = useState<number | null>(null);

  const handleChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    setCode(event.target.value);
  };

  const handleSelect = (event: MouseEvent<HTMLTextAreaElement>) => {
    const target = event.target as HTMLTextAreaElement;
    const start = target.selectionStart;
    const end = target.selectionEnd;
    const selectedText = target.value.substring(start, end);
    const selectedLine = target.value.substring(0, start).split("\n").length;
    setSelectedLine(selectedText ? null : selectedLine);
  };

  useEffect(() => {
    const lines = code.split("\n").length;
    setLineCount(lines);
  }, [code]);

  useEffect(() => {
    const textarea = document.getElementById(
      "code-textarea"
    ) as HTMLTextAreaElement;
    if (textarea) {
      textarea.style.height = "auto";
      textarea.style.height = `${textarea.scrollHeight}px`;
    }
    const lineNumbers = document.getElementById(
      "line-numbers-container"
    ) as HTMLDivElement;
    if (lineNumbers) {
      lineNumbers.style.height = `${textarea.scrollHeight}px`;
    }
  }, [code]);

  return (
    <div className="code-editor">
      <div id="line-numbers-container" className="line-numbers">
        {Array.from(Array(lineCount).keys()).map((index) => (
          <div
            key={index + 1}
            className={`line-number ${
              selectedLine === index + 1 ? "selected" : ""
            }`}
          >
            {index + 1}
          </div>
        ))}
      </div>
      <textarea
        id="code-textarea"
        value={code}
        onChange={handleChange}
        onSelect={handleSelect}
        className="code-editor-textarea"
        placeholder="// Start coding here..."
      />
    </div>
  );
};

export default CodeEditor;
