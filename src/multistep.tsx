import React, { FC, ReactElement, useState } from 'react';

import { Button, Row } from '@geist-ui/react';

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
}> = ({ steps }) => {
  const [currentStep, setCurrentStep] = useState(0);

  const navigateBackward = () => {
    setCurrentStep(Math.max(currentStep - 1, 0));
  };
  const navigateForward = () => {
    setCurrentStep(Math.min(currentStep + 1, steps.length));
    const onNext = steps[currentStep].onNext;
    if (onNext) onNext();
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
        position: 'relative'
      }}
    >
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'flex-start',
          flex: '1 1 auto',
          maxHeight: 'calc(100vh - 5rem)',
          paddingTop: '1rem'
        }}
      >
        {steps[currentStep].component}
      </div>
      <div style={{ margin: '2rem 0', flex: '0 0 3rem' }}>
        <Row justify="space-around">
          <Button
            onClick={navigateBackward}
            style={{ visibility: currentStep === 0 ? 'hidden' : undefined }}
            type="secondary"
            disabled={steps[currentStep].backDisabled}
            ghost
          >
            Back
          </Button>
          <Button
            onClick={navigateForward}
            style={{
              visibility: currentStep + 1 >= steps.length ? 'hidden' : undefined
            }}
            type="success"
            iconRight={steps[currentStep].nextIcon ?? undefined}
            disabled={steps[currentStep].nextDisabled}
            ghost
          >
            {steps[currentStep].nextLabel ?? 'Next'}
          </Button>
        </Row>
      </div>
    </div>
  );
};
