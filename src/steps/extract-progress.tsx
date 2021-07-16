import React, { FC } from 'react';

import { Progress, Text } from '@geist-ui/react';

export const ExtractProgress: FC<{
  extractProgress: number;
  extractMessages: string;
  extractStep: string;
}> = ({ extractProgress, extractMessages, extractStep }) => {
  return (
    <>
      <Progress value={extractProgress} />
      <div style={{ minHeight: '6rem', maxHeight: '6rem' }}>
        {extractStep.split('\n').map((step, i) => (
          <Text key={i}>{step}</Text>
        ))}
      </div>
      <pre
        style={{
          background: 'black',
          color: 'white'
        }}
      >
        {extractMessages}
      </pre>
    </>
  );
};
