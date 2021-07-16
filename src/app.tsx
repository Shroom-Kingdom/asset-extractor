import React, { FC, useEffect, useState } from 'react';

import * as Icon from '@geist-ui/react-icons';
import { invoke } from '@tauri-apps/api/tauri';
import { getCurrent } from '@tauri-apps/api/window';

import { AssetSelect } from './steps/asset-select';
import { Intro } from './steps/intro';
import { MultiStep } from './multistep';
import { ExtractProgress } from './steps/extract-progress';

export const App: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [assetFiles, setAssetFiles] = useState<string[]>([]);
  const [extractProgress, setExtractProgress] = useState<number>(0);
  const [extractMessages, setExtractMessages] = useState<string>('');
  const [extractStep, setExtractStep] = useState<string>('');

  useEffect(() => {
    const current = getCurrent();
    const progressListener = current.listen('extract_progress', event => {
      setExtractProgress(event.payload as number);
    });
    const messageListener = current.listen('extract_message', event => {
      setExtractMessages(extractMessages + event.payload);
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
      setAssetFiles(files);
    } catch (err) {
      console.error(err);
    }
  };

  const handleRemoveFile = (fileName: string) => async () => {
    try {
      const files = await invoke<string[]>('remove_file', { fileName });
      setAssetFiles(files);
    } catch (err) {
      console.error(err);
    }
  };

  const handleStart = async () => {
    try {
      setLoading(true);
      setExtractProgress(0);
      setExtractMessages('');
      setExtractStep('');
      await invoke('extract_assets');
    } catch (err) {
      setExtractProgress(0);
      console.error(err);
    }
    setLoading(false);
  };

  return (
    <MultiStep
      steps={[
        { component: <Intro /> },
        {
          component: (
            <AssetSelect
              loading={loading}
              assetFiles={assetFiles}
              handleAddFiles={handleAddFiles}
              handleRemoveFile={handleRemoveFile}
            />
          ),
          onNext: handleStart,
          nextLabel: 'Start',
          nextIcon: <Icon.PlayCircle />,
          nextDisabled: assetFiles.length === 0
        },
        {
          component: (
            <ExtractProgress
              extractStep={extractStep}
              extractMessages={extractMessages}
              extractProgress={extractProgress}
            />
          ),

          backDisabled: loading
        }
      ]}
    ></MultiStep>
  );
};
