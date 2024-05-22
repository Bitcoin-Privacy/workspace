import React, { useState } from "react";
import QRCode from "react-qr-code";
import { Square } from "@chakra-ui/react";

interface QRCodeGeneratorProps {
  text: string | undefined;
  size: string;
}

const QRCodeGenerator: React.FC<QRCodeGeneratorProps> = ({ text, size }) => {
  return (
    <Square
      size = {size}
      maxBlockSize={"300px"}
      borderColor={"white"}
      borderWidth={"2px"}
    >
      {text && (
        <QRCode
          value={text}
        
          // adjust the size as needed
        />
      )}
    </Square>
  );
};

export default QRCodeGenerator;
