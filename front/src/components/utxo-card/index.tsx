import { UtxoDto } from "@/dtos";
import { Box,Text } from "@chakra-ui/react";


interface UTXOCardProps {
    val : UtxoDto;
    index : number;
}

export function UTXOCard(props : UTXOCardProps){
    const { val , index } = props;

    return (
        <Box
        key={index}
        color="white"
        textAlign="start"
        maxW="80%"
        bg="#3a3a3a"
        p="8px 16px"
        borderRadius="8px"
        >
        <Text noOfLines={1} wordBreak="break-all">
            TxID: {val.txid}
        </Text>
        <Text>Vout: {val.vout}</Text>
        <Text>Value: {val.value}</Text>
        </Box>
    )
}