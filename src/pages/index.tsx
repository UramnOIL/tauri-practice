import React from 'react'
import { NextPage } from 'next'
import styled from '@emotion/styled'
import tw from 'twin.macro'
import { invoke } from 'tauri/api/tauri'

const IndexPage: NextPage = () => {
  const onClick = async (): Promise<void> => {
    await invoke({ cmd: 'myCustomCommand', argument: 'hoge' })
  }
  return <Wrapper onClick={async () => await onClick()}>Hoge</Wrapper>
}

const Wrapper = styled.div`
  ${tw`bg-red-50 h-screen`}
`

export default IndexPage
