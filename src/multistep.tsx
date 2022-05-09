import React, { FC, ReactElement, useState } from 'react';

import { Button, Grid } from '@geist-ui/core';
import { invoke } from '@tauri-apps/api';

export const MultiStep: FC<{
  steps: {
    component: ReactElement;
    backDisabled?: boolean;
    // eslint-disable-next-line @typescript-eslint/ban-types
    onNext?: Function;
    nextLabel?: string;
    nextIcon?: JSX.Element;
    nextDisabled?: boolean;
  }[];
  bundleData: boolean;
}> = ({ steps, bundleData }) => {
  const [currentStep, setCurrentStep] = useState(0);

  const navigateBackward = () => {
    setCurrentStep(Math.max(currentStep - 1, 0));
  };
  const navigateForward = () => {
    setCurrentStep(Math.min(currentStep + 1, steps.length));
    const onNext = steps[currentStep].onNext;
    if (onNext) onNext();
  };

  const download = async () => {
    await invoke('save_bundle_data');
  };

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        width: '750pt',
        maxWidth: '100vw',
        maxHeight: '100vh',
        minHeight: '100vh',
        margin: '0 auto',
        padding: '0 16pt',
        boxSizing: 'border-box',
        position: 'relative',
        overflow: 'hidden'
      }}
    >
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'flex-start',
          flex: '1 0 auto',
          maxHeight: 'calc(100vh - 7rem)',
          paddingTop: '1rem',
          overflow: 'auto'
        }}
        className="flex-no-shrink"
      >
        {steps[currentStep].component}
      </div>
      <Grid.Container
        style={{ margin: '2rem 0', flex: '0 0 3rem' }}
        justify="space-around"
      >
        <Grid>
          <Button
            onClick={navigateBackward}
            style={{
              visibility: currentStep === 0 || bundleData ? 'hidden' : undefined
            }}
            type="secondary"
            disabled={steps[currentStep].backDisabled}
            ghost
          >
            Back
          </Button>
        </Grid>
        <Grid>
          <Button
            onClick={bundleData ? download : navigateForward}
            style={{
              visibility:
                !bundleData && currentStep + 1 >= steps.length
                  ? 'hidden'
                  : undefined
            }}
            type="success"
            iconRight={steps[currentStep].nextIcon ?? undefined}
            disabled={steps[currentStep].nextDisabled}
            ghost
          >
            {bundleData ? 'Save' : steps[currentStep].nextLabel ?? 'Next'}
          </Button>
        </Grid>
      </Grid.Container>
    </div>
  );
};
