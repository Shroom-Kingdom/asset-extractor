import React, { FC, useEffect, useState } from 'react';

import * as Icon from '@geist-ui/react-icons';
import {
  Button,
  Page,
  Progress,
  Row,
  Spacer,
  Spinner,
  Text
} from '@geist-ui/react';
import { invoke } from '@tauri-apps/api/tauri';
import { getCurrent } from '@tauri-apps/api/window';

export const App: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [gameFiles, setGameFiles] = useState<string[]>([]);
  const [extractProgress, setExtractProgress] = useState<number | null>(null);
  const [extractStep, setExtractStep] = useState<string>('');

  useEffect(() => {
    const current = getCurrent();
    const progressListener = current.listen('extract_progress', event => {
      setExtractProgress(event.payload as number);
    });
    const messageListener = current.listen('extract_message', event => {
      console.info(event.payload);
    });
    const stepListener = current.listen('extract_step', event => {
      setExtractStep(event.payload as string);
    });
    return () => {
      (async () => {
        (await progressListener)();
        (await messageListener)();
        (await stepListener)();
      })();
    };
  }, []);

  const handleAddFiles = async () => {
    try {
      const files = await invoke<string[]>('add_files');
      setGameFiles(files);
    } catch (err) {
      console.error(err);
    }
  };

  const handleRemoveFile = (fileName: string) => async () => {
    try {
      const files = await invoke<string[]>('remove_file', { fileName });
      setGameFiles(files);
    } catch (err) {
      console.error(err);
    }
  };

  const handleStart = async () => {
    try {
      setLoading(true);
      setExtractProgress(0);
      setExtractStep('');
      await invoke('extract_assets');
    } catch (err) {
      setExtractProgress(null);
      console.error(err);
    }
    setLoading(false);
  };

  return (
    <>
      <Page>
        <Text>
          This is a simple tool to extract and bundle all assets that are
          required to play on Shroom Kingdom.
        </Text>
        <Spacer y={1} />
        <Text>
          Please select all your game resource files from Super Mario Maker 2:
        </Text>
        <div style={{ maxWidth: '24rem' }}>
          {gameFiles.map(gameFile => (
            <Row
              key={gameFile}
              style={{ marginBottom: '0.6rem' }}
              justify="space-between"
            >
              <span>{gameFile}</span>
              <Button
                auto
                size="mini"
                type="error"
                disabled={loading}
                icon={<Icon.Trash2 />}
                onClick={handleRemoveFile(gameFile)}
              />
            </Row>
          ))}
        </div>
        <Button
          type="success-light"
          disabled={loading}
          iconRight={<Icon.PlusCircle />}
          onClick={handleAddFiles}
        >
          Add
        </Button>
        <Spacer y={2} />
        <Button
          type="success-light"
          disabled={gameFiles.length === 0 || loading}
          iconRight={loading ? <Spinner /> : <Icon.PlayCircle />}
          onClick={handleStart}
        >
          Start
        </Button>
        <Spacer y={2} />
        {extractProgress != null && (
          <>
            <Progress value={extractProgress} />
            {extractStep.split('\n').map((step, i) => (
              <Text key={i}>{step}</Text>
            ))}
          </>
        )}
      </Page>
    </>
  );
};
