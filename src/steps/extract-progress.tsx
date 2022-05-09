import React, { FC } from 'react';

import { Note, Progress, Text } from '@geist-ui/core';

export const ExtractProgress: FC<{
  extractProgress: number;
  extractMessages: string;
  extractStep: string;
  extractError: Record<string, string> | null;
}> = ({ extractProgress, extractMessages, extractStep, extractError }) => (
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
      style={{
        maxWidth: '100%',
        minWidth: '100%',
        whiteSpace: 'break-spaces',
        wordBreak: 'break-word'
      }}
    >
      {extractMessages}
    </pre>
    {extractProgress >= 99.9 && (
      <>
        <Text>
          Please save your file with whatever name you like and with a
          &apos;.tar&apos; extension. You can now go back to{' '}
          <a
            href="https://app.shroomkingdom.net/"
            target="_blank"
            rel="noreferrer"
          >
            Shroom Kingdom
          </a>{' '}
          and load your assets to play!
        </Text>
        <Text>
          Please do not share your assets, if they have been extracted from
          original game files, since they are copyrighted.
        </Text>
      </>
    )}
    {extractError && (
      <Note label="Error" type="error">
        {Object.entries(extractError).map(
          ([key, value]) => `[${key}]: ${value}`
        )}
      </Note>
    )}
  </>
);
