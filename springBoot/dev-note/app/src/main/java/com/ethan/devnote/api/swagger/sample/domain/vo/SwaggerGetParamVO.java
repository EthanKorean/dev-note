package com.ethan.devnote.api.swagger.sample.domain.vo;

import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Schema;
import lombok.Builder;
import lombok.Getter;
import lombok.ToString;

@Getter
@Builder
@ToString
public class SwaggerGetParamVO {
    @Parameter(description = "제목을 입력!")
    private String title;
    @Parameter(description = "내용을 입력!")
    private String desc;
    
}
