import React, { FC, useCallback, useEffect, useRef, useState } from 'react';

import * as Icon from '@geist-ui/react-icons';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { getCurrent } from '@tauri-apps/api/window';

import { AssetSelect } from './steps/asset-select';
import { Intro } from './steps/intro';
import { MultiStep } from './multistep';
import { ExtractProgress } from './steps/extract-progress';

export interface RequiredFilesMissingError {
  RequiredFilesMissing: string[];
}

export const App: FC = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [keys, setKeys] = useState<string[]>([]);
  const [prodKey, setProdKey] = useState<string | null>(null);
  const [assetFiles, setAssetFiles] = useState<string[]>([]);
  const [filesMissing, setFilesMissing] = useState<string[] | null>(null);
  const [extractProgress, setExtractProgress] = useState<number>(0);
  const [extractError, setExtractError] = useState<Record<
    string,
    string
  > | null>(null);
  const [extractMessages, setExtractMessages] = useState<string>('');
  const [extractStep, setExtractStep] = useState<string>('');
  const [bundleData, setBundleData] = useState<boolean>(false);
  const messages = useRef(extractMessages);
  let extractMessageTimeout: number | null = null;

  useEffect(() => {
    const current = getCurrent();
    const progressListener = current.listen('extract_progress', event => {
      setExtractProgress(event.payload as number);
    });
    const messageListener = current.listen('extract_message', event => {
      messages.current = event.payload as string;
      if (!extractMessageTimeout) {
        extractMessageTimeout = setTimeout(() => {
          setExtractMessages(messages.current);
          extractMessageTimeout = null;
        }, 500) as unknown as number;
      }
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

  useEffect(() => {
    const run = async () => {
      try {
        const keys = await invoke<string[]>('find_keys');
        setKeys(keys);
      } catch (err) {
        console.error(err);
      }
    };
    run();
  }, []);

  const handleSetProdKey = useCallback(
    (prodKey: string) => async () => {
      try {
        await invoke('set_prod_key', { prodKey });
        setProdKey(prodKey);
      } catch (err) {
        console.error(err);
      }
    },
    []
  );

  const handleSelectProdKey = useCallback(async () => {
    try {
      const prodKey = await invoke<string>('select_prod_key');
      setProdKey(prodKey);
    } catch (err) {
      console.error(err);
    }
  }, []);

  const handleAddFiles = useCallback(async () => {
    try {
      const selectedFiles = await open({
        multiple: true,
        filters: [{ extensions: ['zip', '7z', 'xci'], name: '.zip,.7z,.xci' }]
      });
      const files = await invoke<string[]>('add_files', {
        files: selectedFiles
      });
      setAssetFiles(files);

      await invoke<string[]>('assert_added_files');
      setFilesMissing(null);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (err: any) {
      if (err.RequiredFilesMissing) {
        const filesMissingError = err as RequiredFilesMissingError;
        setFilesMissing(filesMissingError.RequiredFilesMissing);
      } else {
        console.error(err);
      }
    }
  }, []);

  const handleRemoveFile = useCallback(
    (fileName: string) => async () => {
      try {
        const files = await invoke<string[]>('remove_file', { fileName });
        setAssetFiles(files);

        if (files.length !== 0) {
          await invoke<string[]>('assert_added_files');
        }
        setFilesMissing(null);
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (err: any) {
        if (err.RequiredFilesMissing) {
          const filesMissingError = err as RequiredFilesMissingError;
          setFilesMissing(filesMissingError.RequiredFilesMissing);
        } else {
          console.error(err);
        }
      }
    },
    []
  );

  const handleStart = useCallback(async () => {
    try {
      setLoading(true);
      setExtractProgress(0);
      setExtractError(null);
      setExtractMessages('');
      setExtractStep('');
      await invoke('extract_assets');
      setBundleData(true);
    } catch (err) {
      setExtractProgress(0);
      setExtractError(err as unknown as Record<string, string>);
      console.error(err);
    }
    setLoading(false);
  }, []);

  return (
    <MultiStep
      steps={[
        { component: <Intro /> },
        {
          component: (
            <AssetSelect
              loading={loading}
              keys={keys}
              prodKey={prodKey}
              assetFiles={assetFiles}
              filesMissing={filesMissing}
              handleSetProdKey={handleSetProdKey}
              handleSelectProdKey={handleSelectProdKey}
              handleAddFiles={handleAddFiles}
              handleRemoveFile={handleRemoveFile}
            />
          ),
          onNext: handleStart,
          nextLabel: 'Start',
          nextIcon: <Icon.PlayCircle />,
          nextDisabled:
            assetFiles.length === 0 ||
            (!!assetFiles.find(
              file => file.endsWith('.xci') || file.endsWith('.nsp')
            ) &&
              prodKey == null)
        },
        {
          component: (
            <ExtractProgress
              extractStep={extractStep}
              extractMessages={extractMessages}
              extractProgress={extractProgress}
              extractError={extractError}
            />
          ),

          backDisabled: loading
        }
      ]}
      bundleData={bundleData}
    ></MultiStep>
  );
};
