import React, { useState } from 'react'
import { NextPage } from 'next'
import styled from '@emotion/styled'
import tw from 'twin.macro'
import { promisified } from 'tauri/api/tauri'

interface Image {
  id: string
  format: string
  type: number
  url: string
}

interface Server {
  id: number
  user_id: number
  name: string
  address?: string
  port?: number
  description?: string
  color?: string
  categories: number[]
  web_sites: string[]
  top_image_id?: string
  back_image_id?: string
  is_verified: boolean
  is_archived: boolean
  is_display_server: boolean
  is_display_address: boolean
  is_display_statistics: boolean
  created_at: number
  updated_at: number
  user: {
    id: number
    name: string
    description?: string
    icon_image_id?: string
    created_at: number
    updated_at: number
  }
  top_image: Image
  back_image: Image
  latest_ping: {
    server_id: number
    is_running: boolean
    millisecond?: number
    protocol?: number
    version?: string
    current_player?: number
    max_player?: number
    created_at: number
  }
  yesterday_statistics: {
    date: string
    type: number
    server_id: number
    all_ping_count: number
    valid_ping_count: number
    average_player: number
    max_player: number
    created_at: number
  }
  votes: {
    entire: number
    recently: number
  }
}

interface GetServersResponse {
  servers: Server[]
}

const IndexPage: NextPage = () => {
  const handleClick = async (): Promise<void> => {
    const errorHandler = (reason: any): undefined => {
      setErrorMessage(reason)
      return undefined
    }

    const response = await promisified<GetServersResponse>({
      cmd: 'getServersCommand',
    }).catch(errorHandler)

    if (response !== undefined) {
      setServers(response.servers)
    }

    window.alert('Succeeded!')
  }

  const [servers, setServers] = useState<Server[]>([])
  const [errorMessage, setErrorMessage] = useState('')

  return (
    <Wrapper>
      <Container>
        <Button
          onClick={async () => {
            await handleClick()
          }}
        >
          送信
        </Button>
        {servers.map((server) => (
          <P key={server.id}>{server.name}</P>
        ))}
        <ErrorMsg>{errorMessage}</ErrorMsg>
      </Container>
    </Wrapper>
  )
}

const P = styled.p`
  ${tw`rounded bg-green-300 text-white leading-10`};
`

const ErrorMsg = styled.p`
  ${tw`rounded border-red-900`}
`

const Button = styled.button`
  ${tw`rounded h-auto w-16 bg-blue-500 text-white`};
`

const Wrapper = styled.div`
  ${tw`bg-gray-50`};
`

const Container = styled.div`
  ${tw`bg-red-50 h-screen container m-auto p-4`};
`

export default IndexPage
