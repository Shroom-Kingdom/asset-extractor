import React, { FC, useRef } from 'react';

import { Note, Progress, Text } from '@geist-ui/react';

export const ExtractProgress: FC<{
  extractProgress: number;
  extractMessages: string;
  extractStep: string;
  extractError: Record<string, string> | null;
}> = ({ extractProgress, extractMessages, extractStep, extractError }) => {
  const pre = useRef<HTMLPreElement>(null);
  return (
    <>
      <div style={{ width: '100%', minHeight: '0.625rem' }}>
        <Progress value={extractProgress} type="success" />
      </div>

      <div style={{ minHeight: '6rem', maxHeight: '6rem' }}>
        {extractStep.split('\n').map((step, i) => (
          <Text key={i}>{step}</Text>
        ))}
      </div>

      <pre
        ref={pre}
        style={{
          maxWidth: '100%',
          minWidth: '100%',
          whiteSpace: 'break-spaces',
          wordBreak: 'break-word'
        }}
      >
        {extractMessages}
      </pre>
      {extractError && (
        <Note label="Error" type="error">
          {Object.entries(extractError).map(
            ([key, value]) => `[${key}]: ${value}`
          )}
        </Note>
      )}
    </>
  );
};
