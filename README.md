# GitHub Actionsを学ぶ

RustのかんたんなCLIプログラムを作るプロジェクトをGitHub Actionsでテスト・ビルドを行った後で、バイナリをreleaseにアップロードする
テスト・ビルドを走らせるまでは大して難しくはないが、バイナリをreleaseにアップロードする過程がややこしい。

以下にreleaseにアップロードする部分を抜粋して示す

```YAML
deploy:
  runs-on: ubuntu-latest
  needs: build_and_test

  permissions:
    contents: write

  steps:

    - uses: actions/checkout@v4
    - name: dwonload artifact file
      uses: actions/download-artifact@v4
      with:
        name: build
        path: output/

    - name: deploy to Github Release
      run: gh release upload v1.0.0 output/artifact.tar.gz --clobber
      env:
        GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

ポイントは

- `needs`でこの`deploy`が実行される前に`build_and_test`が実行されるようにする
- `permissions`で書き込み権限を与える
- `env`で` GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}`でトークンを渡す
- `gh`コマンドで`release`へのアップロード処理を実行する

である。
